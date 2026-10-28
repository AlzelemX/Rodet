//@ run-pass
//@ check-run-results
// Rodet: parameter default values, `fn f(x: T = value)`.

fn open(path: &str, read: bool = true, write: bool = false, mode: u32 = 0o644) -> String {
    format!("{path} r={read} w={write} mode={mode:o}")
}

fn greet(name: &str = "world", punct: char = '!') -> String {
    format!("hello {name}{punct}")
}

struct Counter {
    n: i64,
}

impl Counter {
    fn new(start: i64 = 0) -> Self {
        Counter { n: start }
    }

    fn add(&mut self, by: i64 = 1) -> &mut Self {
        self.n += by;
        self
    }
}

struct Wrapper<T> {
    items: Vec<T>,
}

impl<T: std::fmt::Debug> Wrapper<T> {
    fn show(&self, sep: &str = ", ", limit: usize = 10) -> String {
        self.items.iter().take(limit).map(|i| format!("{i:?}")).collect::<Vec<_>>().join(sep)
    }
}

const BIG: u32 = 100;
fn scaled(x: u32, factor: u32 = BIG / 10) -> u32 {
    x * factor
}

fn main() {
    out!("{}", open("a.txt"));
    out!("{}", open("b.txt", write = true));
    out!("{}", open("c.txt", false));
    out!("{}", open("d.txt", mode = 0o600, read = false));

    out!("{}", greet());
    out!("{}", greet("Ali"));
    out!("{}", greet(punct = '?'));

    let mut c = Counter::new();
    c.add().add().add(by = 10);
    out!("{}", c.n);
    out!("{}", Counter::new(start = 5).n);

    let w = Wrapper { items: vec![1, 2, 3, 4] };
    out!("{}", w.show());
    out!("{}", w.show(limit = 2));
    out!("{}", w.show(" | "));

    out!("{}", scaled(3));
}
