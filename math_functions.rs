use std::io::{self, BufRead, BufWriter, StdinLock, StdoutLock, Write};

const MENU: &str = "
╔══════════════════════════════╗
║      MATH FUNCTIONS MENU     ║
╠══════════════════════════════╣
║  1.  Addition                ║
║  2.  Subtraction             ║
║  3.  Multiplication          ║
║  4.  Division                ║
║  5.  Modulus                 ║
║  6.  Power (a^b)             ║
║  7.  Square Root             ║
║  8.  Cube Root               ║
║  9.  Absolute Value          ║
║  10. Floor                   ║
║  11. Ceiling                 ║
║  12. Round                   ║
║  13. Log (base 10)           ║
║  14. Natural Log (ln)        ║
║  15. Log (custom base)       ║
║  16. Sin                     ║
║  17. Cos                     ║
║  18. Tan                     ║
║  19. Factorial               ║
║  20. Is Prime?               ║
║  21. GCD                     ║
║  22. LCM                     ║
║  23. Hypotenuse              ║
║  24. Degrees to Radians      ║
║  25. Radians to Degrees      ║
║  0.  Exit                    ║
╚══════════════════════════════╝
";

const FACT: [u64; 21] = {
    let mut t = [1u64; 21];
    let mut i = 1;
    while i < 21 {
        t[i] = t[i - 1] * i as u64;
        i += 1;
    }
    t
};

const MR_BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

#[inline]
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn pow_mod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mul_mod(r, b, m);
        }
        b = mul_mod(b, b, m);
        e >>= 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &MR_BASES {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'outer: for &a in &MR_BASES {
        let mut x = pow_mod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    loop {
        b >>= b.trailing_zeros();
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            return a << shift;
        }
    }
}

fn lcm(a: u64, b: u64) -> Option<u64> {
    if a == 0 || b == 0 {
        return Some(0);
    }
    (a / gcd(a, b)).checked_mul(b)
}

struct Io {
    inp: StdinLock<'static>,
    out: BufWriter<StdoutLock<'static>>,
    buf: String,
}

macro_rules! say {
    ($io:expr, $($a:tt)*) => {
        let _ = writeln!($io.out, $($a)*);
    };
}

impl Io {
    fn new() -> Self {
        Io {
            inp: io::stdin().lock(),
            out: BufWriter::new(io::stdout().lock()),
            buf: String::with_capacity(64),
        }
    }

    fn line(&mut self, prompt: &str) -> Option<()> {
        let _ = self.out.write_all(prompt.as_bytes());
        let _ = self.out.flush();
        self.buf.clear();
        if self.inp.read_line(&mut self.buf).ok()? == 0 {
            return None;
        }
        Some(())
    }

    fn f64(&mut self, prompt: &str) -> Option<f64> {
        loop {
            self.line(prompt)?;
            if let Ok(v) = self.buf.trim().parse::<f64>() {
                if v.is_finite() {
                    return Some(v);
                }
            }
            say!(self, "Invalid number, try again.");
        }
    }

    fn u64(&mut self, prompt: &str) -> Option<u64> {
        loop {
            self.line(prompt)?;
            if let Ok(v) = self.buf.trim().parse::<u64>() {
                return Some(v);
            }
            say!(self, "Invalid non-negative integer, try again.");
        }
    }

    fn two_f64(&mut self) -> Option<(f64, f64)> {
        let a = self.f64("Enter first number: ")?;
        let b = self.f64("Enter second number: ")?;
        Some((a, b))
    }

    fn two_u64(&mut self) -> Option<(u64, u64)> {
        let a = self.u64("Enter first non-negative integer: ")?;
        let b = self.u64("Enter second non-negative integer: ")?;
        Some((a, b))
    }
}

fn run(io: &mut Io) -> Option<()> {
    say!(io, "Welcome to the Rust Math Functions Calculator!");

    loop {
        let _ = io.out.write_all(MENU.as_bytes());
        io.line("\nEnter your choice: ")?;
        let choice: u32 = match io.buf.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                say!(io, "Invalid choice.");
                continue;
            }
        };
        say!(io, "");

        match choice {
            0 => {
                say!(io, "Goodbye!");
                return Some(());
            }
            1 => {
                let (a, b) = io.two_f64()?;
                say!(io, "Result: {} + {} = {}", a, b, a + b);
            }
            2 => {
                let (a, b) = io.two_f64()?;
                say!(io, "Result: {} - {} = {}", a, b, a - b);
            }
            3 => {
                let (a, b) = io.two_f64()?;
                say!(io, "Result: {} × {} = {}", a, b, a * b);
            }
            4 => {
                let (a, b) = io.two_f64()?;
                if b == 0.0 {
                    say!(io, "Error: Division by zero!");
                } else {
                    say!(io, "Result: {} ÷ {} = {}", a, b, a / b);
                }
            }
            5 => {
                let (a, b) = io.two_f64()?;
                if b == 0.0 {
                    say!(io, "Error: Modulus by zero!");
                } else {
                    say!(io, "Result: {} % {} = {}", a, b, a % b);
                }
            }
            6 => {
                let (a, b) = io.two_f64()?;
                say!(io, "Result: {}^{} = {}", a, b, a.powf(b));
            }
            7 => {
                let a = io.f64("Enter number: ")?;
                if a < 0.0 {
                    say!(io, "Error: Square root of negative number!");
                } else {
                    say!(io, "Result: √{} = {}", a, a.sqrt());
                }
            }
            8 => {
                let a = io.f64("Enter number: ")?;
                say!(io, "Result: ∛{} = {}", a, a.cbrt());
            }
            9 => {
                let a = io.f64("Enter number: ")?;
                say!(io, "Result: |{}| = {}", a, a.abs());
            }
            10 => {
                let a = io.f64("Enter number: ")?;
                say!(io, "Result: floor({}) = {}", a, a.floor());
            }
            11 => {
                let a = io.f64("Enter number: ")?;
                say!(io, "Result: ceil({}) = {}", a, a.ceil());
            }
            12 => {
                let a = io.f64("Enter number: ")?;
                say!(io, "Result: round({}) = {}", a, a.round());
            }
            13 => {
                let a = io.f64("Enter number: ")?;
                if a <= 0.0 {
                    say!(io, "Error: log10 undefined for <= 0!");
                } else {
                    say!(io, "Result: log10({}) = {}", a, a.log10());
                }
            }
            14 => {
                let a = io.f64("Enter number: ")?;
                if a <= 0.0 {
                    say!(io, "Error: ln undefined for <= 0!");
                } else {
                    say!(io, "Result: ln({}) = {}", a, a.ln());
                }
            }
            15 => {
                let a = io.f64("Enter number: ")?;
                let base = io.f64("Enter base: ")?;
                if a <= 0.0 || base <= 0.0 || base == 1.0 {
                    say!(io, "Error: Invalid input for logarithm!");
                } else {
                    say!(io, "Result: log_{}({}) = {}", base, a, a.log(base));
                }
            }
            16 => {
                let a = io.f64("Enter angle in degrees: ")?;
                say!(io, "Result: sin({}°) = {}", a, a.to_radians().sin());
            }
            17 => {
                let a = io.f64("Enter angle in degrees: ")?;
                say!(io, "Result: cos({}°) = {}", a, a.to_radians().cos());
            }
            18 => {
                let a = io.f64("Enter angle in degrees: ")?;
                say!(io, "Result: tan({}°) = {}", a, a.to_radians().tan());
            }
            19 => {
                let n = io.u64("Enter a non-negative integer: ")?;
                if n > 20 {
                    say!(io, "Error: Enter an integer between 0 and 20.");
                } else {
                    say!(io, "Result: {}! = {}", n, FACT[n as usize]);
                }
            }
            20 => {
                let n = io.u64("Enter a non-negative integer: ")?;
                say!(io, "Result: {} is {}prime", n, if is_prime(n) { "" } else { "not " });
            }
            21 => {
                let (a, b) = io.two_u64()?;
                say!(io, "Result: GCD({}, {}) = {}", a, b, gcd(a, b));
            }
            22 => {
                let (a, b) = io.two_u64()?;
                match lcm(a, b) {
                    Some(v) => say!(io, "Result: LCM({}, {}) = {}", a, b, v),
                    None => say!(io, "Error: LCM overflows u64!"),
                }
            }
            23 => {
                let (a, b) = io.two_f64()?;
                say!(io, "Result: hypotenuse({}, {}) = {}", a, b, a.hypot(b));
            }
            24 => {
                let a = io.f64("Enter degrees: ")?;
                say!(io, "Result: {}° = {} radians", a, a.to_radians());
            }
            25 => {
                let a = io.f64("Enter radians: ")?;
                say!(io, "Result: {} radians = {}°", a, a.to_degrees());
            }
            _ => {
                say!(io, "Invalid choice, please select from the menu.");
            }
        }
    }
}

fn main() {
    let mut io = Io::new();
    run(&mut io);
    let _ = io.out.flush();
}
