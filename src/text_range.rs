pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

impl TextRange {
    pub fn new(start: usize, end: usize) -> TextRange {
        TextRange {
            start,
            end: if end > start { end } else { start },
        }
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn fill(&self, with: char) -> String {
        (with.to_string()).repeat(self.len())
    }
}
