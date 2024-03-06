#[derive(Debug, Clone, Copy)]
pub struct StringReader<'a> {
    pub pos: usize,
    pub line: u32,
    pub src: &'a str,
}

impl<'a> StringReader<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            pos: 0,
            line: 1,
            src,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.src.len()
    }

    pub fn ln(&mut self) {
        self.line += 1;
    }

    pub fn read(&mut self) -> char {
        let val = self.src.chars().nth(self.pos).unwrap_or('\0');
        self.pos += 1;
        val
    }

    pub fn peek(&self) -> char {
        self.src.chars().nth(self.pos).unwrap_or('\0')
    }
}
