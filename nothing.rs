use std::time::Duration;

fn greet(n: u32) -> String {
    format!("tick {n}\n")
}

fn main() {
    let mut count = 0_u32;
    loop {
        let msg = greet(count);
        print!("{msg}");
        count += 1;
        std::thread::sleep(Duration::from_millis(800));
    }
}
