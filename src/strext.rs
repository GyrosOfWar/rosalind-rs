use std::str::from_utf8;
use std::slice::{Chunks, Windows};

pub struct StrWindows<'a> {
    v: &'a str,
    size: usize
}

impl<'a> Iterator for StrWindows<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        if self.size > self.v.len() {
            None
        } else {
            let ret = Some(&self.v[..self.size]);
            self.v = &self.v[1..];
            ret
        } 
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.size > self.v.len() {
            (0, Some(0))
        } else {
            let size = self.v.len() - self.size + 1;
            (size, Some(size))
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
            v: &self,
            size: size
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
