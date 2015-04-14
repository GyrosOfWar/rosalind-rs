#![allow(dead_code)]

use std::io::prelude::*;
use std::collections::{HashMap, HashSet};
use std::str::from_utf8_unchecked;
use strext::StrExt;

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
pub fn parse_fasta(data: &str) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
   // let file = BufReader::new(File::open(Path::new(path)).unwrap());
    let mut saved_label = String::new();
    for line in data.lines() {
        if line.starts_with(">") {
            let label = line[1..].to_string();
            result.insert(label.clone(), String::new());
            saved_label = label;
        } else {
            if result.contains_key(&saved_label) {
                let p = result.get_mut(&saved_label).unwrap();
                p.push_str(&line);
            } else {
                result.insert(saved_label.clone(), line.to_string());
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
        for codon in string.chunks(3) {
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
// TODO create DNAString and ProteinString structs
pub fn translate_exons(dna: &str, introns: Vec<&str>, dna_codon_table: HashMap<&str, char>) -> String {
    // Maybe a one pass solution is also possible
    let mut exons = String::new();
    exons.push_str(dna);
    for intron in introns {
        exons = exons.replace(intron, "");
    }
    let mut translated = String::new();
    
    for codon in exons.chunks(3) {
        match dna_codon_table.get(codon) {
            Some(&acid) =>{
                // Stop codon
                if acid == '_' {
                    return translated;
                }
                translated.push(acid);
            },
            None => {}
        }
    }
    
    translated
}

pub fn find_subsequence(haystack: &str, needle: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut needle_iter = needle.chars().peekable();
 
    for (i, ch) in haystack.chars().enumerate() {
        match needle_iter.peek() {
            Some(&nc) => {
                if nc == ch {
                    needle_iter.next();
                    indices.push(i+1);
                }
            }
            None => return indices
        }
    }
    
    indices
}
