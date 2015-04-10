#![allow(dead_code)]

use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::str::from_utf8_unchecked;

/// Counts the number of occurrences of the characters A, G, C and T in the input string.
pub fn count_nucleotides(input: &str) -> (usize, usize, usize, usize) {
    input.chars()
        .fold((0, 0, 0, 0),
              |(a, g, c, t), ch| {
                  match ch {
                      'A' => (a+1, g, c, t),
                      'G' => (a, g+1, c, t),
                      'C' => (a, g, c+1, t),
                      'T' => (a, g, c, t+1),
                      _ => (a, g, c, t)
                  }
              })
}
/// Returns a table mapping codons (three nucleotides) to proteins.
/// The stop codons maps to the character '_'.
pub fn dna_codon_table() -> HashMap<&'static str, char> {
    let data = include_str!("../dna_codon_table.txt");
    let mut table = HashMap::new();
    for line in data.lines() {
        let split: Vec<_> = line.split(' ').collect();
        let codon = split[0].trim();
        let ch = split[1].trim();
        if ch == "Stop" {
            table.insert(codon, '_');
        } else {
            table.insert(codon, ch.chars().nth(0).unwrap());
        }
    }
    
    table
}

pub fn dna_to_rna(input: &str) -> String {
    input.replace("T", "U")
}
/// Returns the reverse complement of a string.
pub fn reverse_complement(input: &str) -> String {
    input.chars().rev().map(|ch| match ch {
        'A' => 'T',
        'C' => 'G',
        'T' => 'A',
        'G' => 'C',
        c => c
    }).collect()
}             
/// Parses the FASTA file at the given path and returns
/// the data stored in it as a map from label to data point.
pub fn parse_fasta(path: &str) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    let file = BufReader::new(File::open(Path::new(path)).unwrap());
    let mut saved_label = String::new();
    for l in file.lines() {
        let tmp = l.ok().unwrap();
        let line = tmp.trim().to_string();

        if line.starts_with(">") {
            let label = line[1..].to_string();
            result.insert(label.clone(), String::new());
            saved_label = label;
        } else {
            if result.contains_key(&saved_label) {
                let p = result.get_mut(&saved_label).unwrap();
                p.push_str(&line);
            } else {
                result.insert(saved_label.clone(), line);
            }
        }
    }
    
    result
}

pub fn gc_content(input: &str) -> f64 {
    let mut gc_count = 0;

    for ch in input.chars() {
        match ch {
            'G' | 'C' => gc_count += 1,
            _ => {}
        }
    }

    let p = (gc_count as f64) / (input.len() as f64);
    p * 100.0
}

pub fn hamming_distance(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .fold(0, |count, (a, b)|
              if a != b { count + 1 }
              else { count })
}

pub fn protein_weights(protein_string: &str, table: HashMap<char, f64>) -> f64 {
    protein_string
        .chars()
        .map(|c| table[&c])
        .fold(0.0, |sum, x| sum + x)
}
/// Translates a reading frame from the given DNA string to a protein string. If
/// the given string has no start codon or does not have an end codon anywhere,
/// returns None. 
pub fn reading_frame(string: &str, dna_codon_table: &HashMap<&str, char>) -> Option<String> {
    if !string.starts_with("ATG") {
        None
    } else {
        let mut value = String::new();
        for codon_bytes in string.as_bytes().chunks(3) {
            let codon = unsafe { from_utf8_unchecked(codon_bytes) };

            if let Some(&dna) = dna_codon_table.get(codon) {
                // Stop codon
                if dna == '_' {
                    return Some(value);
                }
                value.push(dna);
            }
        }
        // No stop codon
        None
    }
}
/// Finds all the open reading frames in the given DNA string and returns them in a set. 
pub fn open_reading_frames(dna_string: &str, dna_codon_table: HashMap<&str, char>) -> HashSet<String> {
    let mut result = HashSet::new();
    let reverse_complement = reverse_complement(dna_string);
    for start in 0..dna_string.len() - 3 {
        if let Some(s) = reading_frame(&dna_string[start..], &dna_codon_table) {
            result.insert(s);
        }
        if let Some(s) = reading_frame(&reverse_complement[start..], &dna_codon_table) {
            result.insert(s);
        }
    }

    result
}

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

    fn peek_char(&self) -> char {
        self.source.chars().next().unwrap()
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
    
    pub fn parse(&mut self) -> Vec<Motif> {
        while !self.eof() {
            let current = self.consume_char();
            let token = match current {
                '[' => self.parse_either(),
                '{' => self.parse_not(),
                _ => Motif::Char(current)
            };
            self.motif.push(token);
        }

        self.motif.clone()
    }
}
#[derive(Debug, Clone)]
pub enum Motif {
    Char(char),
    Either(Vec<char>),
    Not(char)
}


pub fn find_protein_motif(motif: &str, data: &str) -> Vec<usize> {
    unimplemented!()
}
