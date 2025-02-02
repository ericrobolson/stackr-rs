use std::{fmt::Display, path::PathBuf};

/// Location of a token/word in the source code.
#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    /// Line number.
    pub line: usize,
    /// Column number.
    pub column: usize,
    /// File name.
    pub path: Option<PathBuf>,
}
impl Location {
    pub fn new(path: Option<PathBuf>) -> Self {
        Location {
            line: 1,
            column: 1,
            path,
        }
    }

    pub fn next(&mut self) {
        self.column += 1;
    }

    pub fn new_line(&mut self) {
        self.column = 0;
        self.line += 1;
    }
}
impl Default for Location {
    fn default() -> Self {
        Location {
            line: 1,
            column: 1,
            path: None,
        }
    }
}
impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.path {
            Some(path) => write!(f, "{}:{}:{}", path.display(), self.line, self.column),
            None => write!(f, "{}:{}:{}", "stdin", self.line, self.column),
        }
    }
}
