
mod add_two_numbers;
mod longest_substring;
fn main() {
    println!("{}", mk_slice("foo bar baz"));
}

fn mk_slice(string: &str) -> &str {
    &string[2..3]

}