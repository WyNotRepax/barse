use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    range: Range<usize>,
}

impl Span {
    pub fn new(start: usize, len: usize) -> Self {
        Self {
            range: start..start + len,
        }
    }

    pub fn fetch<'a>(&self, code: &'a str) -> &'a str {
        &code[self.range.clone()]
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self { range }
    }
}
