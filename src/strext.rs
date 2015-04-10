use std::str::from_utf8;
use std::slice::{Chunks, Windows};

pub struct StrWindows<'a> {
    iter: Windows<'a, u8>
}

impl<'a> Iterator for StrWindows<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        match self.iter.next() {
            Some(w) => from_utf8(w).ok(),
            None => None
        }
    }
}

pub struct StrChunks<'a> {
    iter: Chunks<'a, u8>
}

impl<'a> Iterator for StrChunks<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        match self.iter.next() {
            Some(c) => from_utf8(c).ok(),
            None => None
        }
    }
}

pub trait StrExt {
    fn windows(&self, size: usize) -> StrWindows;
    fn chunks(&self, size: usize) -> StrChunks;
}

impl StrExt for str {
    fn windows(&self, size: usize) -> StrWindows {
        StrWindows {
            iter: self.as_bytes().windows(size)
        }
    }

    fn chunks(&self, size: usize) -> StrChunks {
        StrChunks {
            iter: self.as_bytes().chunks(size)
        }
    }
}

mod tests {
    use super::StrExt;
    
    #[test]
    fn basic_windows() {
        let s = "ABCD";
        let expected = vec!["AB", "BC", "CD"];
        let actual: Vec<&str> = s.windows(2).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn basic_chunks() {
        let s = "ATGATGATG";
        let expected = vec!["ATG", "ATG", "ATG"];
        let actual: Vec<&str> = s.chunks(3).collect();

        assert_eq!(expected, actual);
    }
}
