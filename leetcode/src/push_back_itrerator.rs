use std::str::Chars;

pub struct PushBackChars<'a> {
    iter: Chars<'a>,
    stack: Vec<char>,
}
impl PushBackChars<'_> {
    pub fn new(chars: Chars) -> PushBackChars {
        PushBackChars{iter: chars, stack: Vec::new() }
    }
    pub fn push(&mut self, c: char) {
        self.stack.push(c);
    }
}
impl Iterator for PushBackChars<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().or_else(|| self.iter.next())
    }
}

#[cfg(test)]
mod tests {
    use crate::push_back_itrerator::PushBackChars;
    #[test]
    fn test() {
        let str = "hello world";
        let mut pbi = PushBackChars::new(str.chars());
        assert_eq!(str, pbi.collect::<String>());

        pbi = PushBackChars::new(str.chars());
        assert_eq!(Some('h'), pbi.next());
        assert_eq!(Some('e'), pbi.next());
        pbi.push('x');
        assert_eq!(Some('x'), pbi.next());
        assert_eq!(Some('l'), pbi.next());
        pbi.push('y');
        pbi.push('z');
        assert_eq!(Some('z'), pbi.next());
        assert_eq!(Some('y'), pbi.next());
        assert_eq!(Some('l'), pbi.next());
    }
}