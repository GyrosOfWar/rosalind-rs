#![allow(dead_code)]

use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

pub fn count_nucleotides(input: &str) -> (usize, usize, usize, usize) {
    let mut a_count = 0;
    let mut g_count = 0;
    let mut c_count = 0;
    let mut t_count = 0;
    
    for ch in input.chars() {
        match ch {
            'A' => a_count += 1,
            'G' => g_count += 1,
            'C' => c_count += 1,
            'T' => t_count += 1,
            '\n' | '\t' | ' ' | '\r' => {},
            p => panic!("Invalid character: {}", p)
        }
    }

    (a_count, c_count, g_count, t_count)
}

pub fn dna_to_rna(input: &str) -> String {
    input.replace("T", "U")
}

pub fn reverse_complement(input: &str) -> String {
    input.chars().rev().map(|ch| match ch {
        'A' => 'T',
        'C' => 'G',
        'T' => 'A',
        'G' => 'C',
        c => c
    }).collect()
}             

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
    protein_string.chars().map(|c| table[&c]).fold(0.0, |sum, x| sum + x)
}

pub fn open_reading_frames(dna_string: &str, dna_codon_table: HashMap<&str, char>) -> Vec<String> {
    for i in 0..dna_string.len()-3 {
        let codon = &dna_string[i..i+3];
        println!("{}", codon);
    }

    let reverse_complement = reverse_complement(dna_string);
    
    vec![]
}
