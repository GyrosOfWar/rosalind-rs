use std::cmp;

/// An iterator over overlapping substrings of length `size`.
/// Implementation details are pretty much identical to
/// std::slice::Windows. 
#[derive(Debug, Clone)]
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

/// An iterator over non-overlapping substrings of
/// length `size`. As with StrWindows, the implementation
/// is pretty much identical to std::slice::Chunks.
pub struct StrChunks<'a> {
    v: &'a str,
    size: usize
}

impl<'a> Iterator for StrChunks<'a> {
    type Item = &'a str;
    
    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        if self.v.len() == 0 {
            None
        } else {
            let ch = cmp::min(self.v.len(), self.size);
            let (fst, snd) = (&self.v[..ch], &self.v[ch..]);
            self.v = snd;
            Some(fst)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.v.len() == 0 {
            (0, Some(0))
        } else {
            let n = self.v.len() / self.size;
            let rem = self.v.len() % self.size;
            let n = if rem > 0 { n+1 } else { n };
            (n, Some(n))
        }
    }
}

impl<'a> DoubleEndedIterator for StrChunks<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a str> {
        if self.v.len() == 0 {
            None
        } else {
            let remainder = self.v.len() % self.size;
            let chunksz = if remainder != 0 { remainder } else { self.size };
            let split = self.v.len() - chunksz;
            let (fst, snd) = (&self.v[..split], &self.v[split..]);
            self.v = fst;
            Some(snd)
        }
    }
}

impl <'a> ExactSizeIterator for StrChunks<'a> {}

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
            v: &self,
            size: size
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
