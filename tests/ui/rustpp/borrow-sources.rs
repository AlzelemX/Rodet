//@ run-pass
//@ check-run-results
// Rust++: `&{x, y} T` in a return type names the parameters the reference
// borrows from, instead of a lifetime.

fn longest(x: &str, y: &str) -> &{x, y} str {
    if x.len() >= y.len() { x } else { y }
}

// Only `v` is a source; `n` can be dropped right after the call.
fn pick(v: &Vec<i32>, n: &usize) -> &{v} i32 {
    &v[*n]
}

fn split_at_space(text: &mut String) -> (&{text} str, usize) {
    text.push('!');
    let i = text.find(' ').unwrap_or(text.len());
    (&text[..i], i)
}

struct Library {
    books: Vec<String>,
}

impl Library {
    fn first(&self) -> &{self} str {
        self.books.first().map(|s| s.as_str()).unwrap_or("none")
    }

    fn either(&self, other: &str, use_other: bool) -> &{self, other} str {
        if use_other { other } else { &self.books[0] }
    }
}

fn get<T>(items: &[T], i: usize) -> Option<&{items} T> {
    items.get(i)
}

fn main() {
    let a = String::from("hello");
    let r;
    {
        let b = String::from("hi");
        r = longest(&a, &b);
        out!("{r}");
    }

    let v = vec![10, 20, 30];
    let x = {
        let n = 1;
        pick(&v, &n)
    };
    out!("{x}");

    let mut t = String::from("hello world");
    let (word, i) = split_at_space(&mut t);
    out!("{word} {i}");

    let lib = Library { books: vec!["Dune".to_string()] };
    out!("{}", lib.first());
    out!("{}", lib.either("other", true));

    out!("{:?}", get(&[1, 2, 3], 2));
}
