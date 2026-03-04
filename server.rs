use std::{
    collections::HashMap,
    fmt,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

// ─── Thread Pool ──────────────────────────────────────────────────────────────

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();
        ThreadPool { workers, sender }
    }

    fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.sender.send(Box::new(f)).expect("Thread pool send failed");
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    println!("[worker {id}] handling request");
                    job();
                }
                Err(_) => {
                    println!("[worker {id}] shutting down");
                    break;
                }
            }
        });
        Worker { id, thread: Some(thread) }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(t) = worker.thread.take() {
                t.join().ok();
            }
        }
    }
}

// ─── HTTP ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

struct Response {
    status: u16,
    reason: &'static str,
    content_type: &'static str,
    body: String,
}

impl Response {
    fn ok(body: impl Into<String>, content_type: &'static str) -> Self {
        Self { status: 200, reason: "OK", content_type, body: body.into() }
    }
    fn not_found() -> Self {
        Self { status: 404, reason: "Not Found", content_type: "text/plain", body: "404 Not Found".into() }
    }
    fn method_not_allowed() -> Self {
        Self { status: 405, reason: "Method Not Allowed", content_type: "text/plain", body: "405 Method Not Allowed".into() }
    }

    fn send(self, stream: &mut TcpStream) {
        let bytes = self.body.len();
        let raw = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            self.status, self.reason, self.content_type, bytes, self.body
        );
        stream.write_all(raw.as_bytes()).ok();
    }
}

fn parse_request(stream: &TcpStream) -> Option<Request> {
    let mut reader = BufReader::new(stream);
    let mut first_line = String::new();
    reader.read_line(&mut first_line).ok()?;
    let mut parts = first_line.trim().splitn(3, ' ');
    let method = parts.next()?.to_string();
    let path = parts.next()?.to_string();

    let mut headers = HashMap::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).ok()?;
        let line = line.trim().to_string();
        if line.is_empty() { break; }
        if let Some((k, v)) = line.split_once(':') {
            headers.insert(k.trim().to_lowercase(), v.trim().to_string());
        }
    }

    let body = if let Some(len) = headers.get("content-length").and_then(|v| v.parse::<usize>().ok()) {
        let mut buf = vec![0u8; len];
        use std::io::Read;
        reader.read_exact(&mut buf).ok()?;
        String::from_utf8_lossy(&buf).into_owned()
    } else { String::new() };

    Some(Request { method, path, headers, body })
}

// ─── State ────────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct AppState {
    start: Instant,
    hits: Arc<Mutex<u64>>,
    notes: Arc<Mutex<Vec<(u64, String)>>>,  // (timestamp, text)
}

impl AppState {
    fn new() -> Self {
        Self {
            start: Instant::now(),
            hits: Arc::new(Mutex::new(0)),
            notes: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn uptime(&self) -> Duration { self.start.elapsed() }
    fn inc_hits(&self) -> u64 { let mut h = self.hits.lock().unwrap(); *h += 1; *h }
    fn add_note(&self, text: String) {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        self.notes.lock().unwrap().push((ts, text));
    }
    fn get_notes(&self) -> Vec<(u64, String)> { self.notes.lock().unwrap().clone() }
}

// ─── Router ───────────────────────────────────────────────────────────────────

fn router(req: &Request, state: &AppState) -> Response {
    let hits = state.inc_hits();
    let path = req.path.split('?').next().unwrap_or("/");

    match (req.method.as_str(), path) {
        ("GET", "/") => Response::ok(home_page(state, hits), "text/html"),
        ("GET", "/status") => Response::ok(status_json(state, hits), "application/json"),
        ("GET", "/notes") => Response::ok(notes_page(state), "text/html"),
        ("POST", "/notes") => {
            if req.body.trim().is_empty() {
                Response::ok("<p>Empty note ignored.</p>", "text/html")
            } else {
                state.add_note(req.body.trim().to_string());
                Response::ok(notes_page(state), "text/html")
            }
        }
        ("GET", _) => Response::not_found(),
        _ => Response::method_not_allowed(),
    }
}

// ─── HTML Views ───────────────────────────────────────────────────────────────

fn html_shell(title: &str, body: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <title>{title}</title>
  <style>
    *{{box-sizing:border-box;margin:0;padding:0}}
    body{{font-family:'Segoe UI',system-ui,sans-serif;background:#0f172a;color:#e2e8f0;min-height:100vh;padding:2rem}}
    h1{{color:#38bdf8;margin-bottom:.5rem}}
    h2{{color:#7dd3fc;margin:1.5rem 0 .75rem}}
    nav a{{color:#94a3b8;text-decoration:none;margin-right:1.5rem;font-size:.9rem}}
    nav a:hover{{color:#38bdf8}}
    .card{{background:#1e293b;border:1px solid #334155;border-radius:12px;padding:1.5rem;margin-top:1.5rem}}
    .badge{{display:inline-block;background:#0ea5e9;color:#fff;border-radius:99px;padding:2px 10px;font-size:.75rem;font-weight:600}}
    .stat{{display:flex;justify-content:space-between;padding:.4rem 0;border-bottom:1px solid #334155}}
    .stat:last-child{{border:none}}
    form{{display:flex;gap:.5rem;margin-top:.75rem}}
    input[type=text]{{flex:1;background:#0f172a;border:1px solid #475569;color:#e2e8f0;border-radius:6px;padding:.5rem .75rem;font-size:.95rem}}
    input[type=text]:focus{{outline:none;border-color:#38bdf8}}
    button{{background:#0ea5e9;color:#fff;border:none;border-radius:6px;padding:.5rem 1.2rem;cursor:pointer;font-weight:600}}
    button:hover{{background:#0284c7}}
    .note{{background:#0f172a;border-left:3px solid #38bdf8;border-radius:4px;padding:.6rem .9rem;margin-bottom:.5rem;font-size:.9rem}}
    .note time{{color:#64748b;font-size:.75rem;display:block;margin-bottom:.2rem}}
    code{{background:#334155;padding:2px 6px;border-radius:4px;font-size:.85rem}}
  </style>
</head>
<body>
  <nav style="margin-bottom:1.5rem">
    <a href="/">🏠 Home</a>
    <a href="/notes">📝 Notes</a>
    <a href="/status">📊 JSON Status</a>
  </nav>
  {body}
</body>
</html>"#)
}

fn home_page(state: &AppState, hits: u64) -> String {
    let up = state.uptime();
    let secs = up.as_secs();
    let (h, m, s) = (secs / 3600, (secs % 3600) / 60, secs % 60);
    let body = format!(r#"
<h1>⚙️ Rust HTTP Server</h1>
<p style="color:#64748b;margin-bottom:.5rem">Zero dependencies · std-only · multi-threaded</p>
<span class="badge">RUNNING</span>

<div class="card">
  <h2>Server Stats</h2>
  <div class="stat"><span>Uptime</span><strong>{h:02}:{m:02}:{s:02}</strong></div>
  <div class="stat"><span>Total Requests</span><strong>{hits}</strong></div>
  <div class="stat"><span>Worker Threads</span><strong>4</strong></div>
  <div class="stat"><span>Bind Address</span><code>127.0.0.1:7878</code></div>
</div>

<div class="card">
  <h2>Routes</h2>
  <div class="stat"><code>GET /</code><span>This page</span></div>
  <div class="stat"><code>GET /notes</code><span>View &amp; post notes</span></div>
  <div class="stat"><code>POST /notes</code><span>Add a note (body = text)</span></div>
  <div class="stat"><code>GET /status</code><span>JSON health check</span></div>
</div>"#);
    html_shell("Rust Server", &body)
}

fn notes_page(state: &AppState) -> String {
    let notes = state.get_notes();
    let list = if notes.is_empty() {
        "<p style='color:#64748b'>No notes yet.</p>".to_string()
    } else {
        notes.iter().rev().map(|(ts, text)| {
            format!("<div class='note'><time>unix:{ts}</time>{}</div>", html_escape(text))
        }).collect::<Vec<_>>().join("\n")
    };
    let body = format!(r#"
<h1>📝 Notes</h1>
<div class="card">
  <h2>Add a Note</h2>
  <form method="POST" action="/notes">
    <input type="text" name="note" placeholder="Type something…" autocomplete="off">
    <button type="submit">Post</button>
  </form>
</div>
<div class="card">
  <h2>All Notes ({} total)</h2>
  {list}
</div>"#, notes.len());
    html_shell("Notes — Rust Server", &body)
}

fn status_json(state: &AppState, hits: u64) -> String {
    let up = state.uptime().as_secs();
    let note_count = state.notes.lock().unwrap().len();
    format!(r#"{{"status":"ok","uptime_secs":{up},"total_requests":{hits},"note_count":{note_count},"threads":4}}"#)
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    let addr = "127.0.0.1:7878";
    let listener = TcpListener::bind(addr).expect("Failed to bind");
    let pool = ThreadPool::new(4);
    let state = AppState::new();

    println!("🦀 Rust HTTP Server");
    println!("   Listening on http://{addr}");
    println!("   Press Ctrl+C to stop\n");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let state = state.clone();
                pool.execute(move || {
                    stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
                    if let Some(req) = parse_request(&stream) {
                        println!("  {} {}", req.method, req.path);
                        let response = router(&req, &state);
                        response.send(&mut stream);
                    }
                });
            }
            Err(e) => eprintln!("Connection error: {e}"),
        }
    }
}
