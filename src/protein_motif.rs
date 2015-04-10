use std::str::from_utf8_unchecked;

pub struct ProteinMotif<'a> {
    source: &'a str,
    motif: Vec<Motif>,
    idx: usize
}

impl<'a> ProteinMotif<'a> {
    pub fn new(src: &'a str) -> ProteinMotif<'a> {
        ProteinMotif {
            source: src,
            motif: vec![],
            idx: 0
        }
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.source[self.idx..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.idx += next_pos;
        return cur_char;
    }

    fn eof(&self) -> bool {
        self.idx >= self.source.len()
    }

    fn parse_either(&mut self) -> Motif {
        let mut chars = Vec::new();
        let limit = 5;
        for _ in 0..limit {
            match self.consume_char() {
                ']' => return Motif::Either(chars),
                c => chars.push(c) 
            }
        }

        panic!("Invalid either string")
    }

    fn parse_not(&mut self) -> Motif {
        let ch = self.consume_char();
        assert!(self.consume_char() == '}');
        Motif::Not(ch)
    }
    
    pub fn parse(&mut self) {
        while !self.eof() {
            let current = self.consume_char();
            let token = match current {
                '[' => self.parse_either(),
                '{' => self.parse_not(),
                _ => Motif::Char(current)
            };
            self.motif.push(token);
        }
    }

    fn is_match(&self, window: &str) -> bool {
        for (token, c) in self.motif.iter().zip(window.chars()) {
            match *token {
                Motif::Char(d) => if c != d {
                    return false;
                },
                Motif::Either(ref chars) => if !chars.contains(&c) {
                    return false;
                },
                Motif::Not(d) => if c == d {
                    return false;
                }
            }
        }
    
        true
    }

    pub fn find_motif(&self, data: &str) -> Vec<usize> {
        let len = self.motif.len();
        let mut indices = Vec::new();
        for (i, w) in data.as_bytes().windows(len).enumerate() {
            let window = unsafe { from_utf8_unchecked(w) };
            if self.is_match(window) {
                indices.push(i + 1);
            }
        }

        indices
    }
}
#[derive(Debug, Clone)]
pub enum Motif {
    Char(char),
    Either(Vec<char>),
    Not(char)
}

