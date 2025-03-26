fn main() {
}

struct T {
    data: u8,
    children: Option<Box<(T, T)>>,
}

fn traverse(s: &T) {
    match &s.children {
        None => {}, // TODO: do something
        Some(ch) => {
            traverse(&ch.0);
            traverse(&ch.1);
        }
    }
}
