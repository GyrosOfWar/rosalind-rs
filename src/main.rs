use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

fn count_nucleotides(input: &str) -> (usize, usize, usize, usize) {
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

fn dna_to_rna(input: &str) -> String {
    input.replace("T", "U")
}

fn reverse_complement(input: &str) -> String {
    let mut result = String::new();

    for ch in input.chars().rev() {
        match ch {
            'A' => result.push('T'),
            'C' => result.push('G'),
            'T' => result.push('A'),
            'G' => result.push('C'),
            '\n' | '\t' | ' ' | '\r' => {},
            c => panic!("Invalid character: {}", c)
        }
    }

    result
}

fn fib_rec(n: i64, k: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_rec(n - 1, k) + k * (fib_rec(n - 2, k))
    }
}

fn parse_fasta(path: &str) -> HashMap<String, String> {
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


#[derive(PartialEq, Eq)]
pub struct ProfileMatrix {
    data: [Vec<usize>; 4]
}

impl ProfileMatrix {
    pub fn new(data: Vec<String>) -> ProfileMatrix {
        let size = data[0].len();
        let mut result = [Vec::with_capacity(size),
                      Vec::with_capacity(size),
                      Vec::with_capacity(size),
                      Vec::with_capacity(size)];

        let col_data = transpose(data.iter().map(|s| s.chars().collect()).collect());
            
        for col in col_data {
            let (a, c, g, t) = count_nucleotides(&vec_to_string(col));
            result[0].push(a);
            result[1].push(c);
            result[2].push(g);
            result[3].push(t);
        }

        ProfileMatrix {
            data: result
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Nucleotide {
    A = 0, C = 1, G = 2, T = 3
}
use Nucleotide::{A, C, G, T};

impl std::ops::Index<Nucleotide> for ProfileMatrix {
    type Output = Vec<usize>;
    fn index(&self, idx: Nucleotide) -> &Vec<usize> {
        match idx {
            A => &self.data[0],
            C => &self.data[1],
            G => &self.data[2],
            T => &self.data[3]
        }
    }
}
use std::fmt;

impl fmt::Debug for ProfileMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for n in vec![A, C, G, T] {
            result.push_str(&format!("{:?}: {}\n", n, format_numbers(&self[n])));
        }
        write!(f, "{}", result)
    }
}

fn transpose<T: Clone>(data: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = data[0].len();
    let mut tr_data = Vec::new();
    for i in 0..len {
        let mut col = Vec::new();
        for v in data.iter() {
            col.push(v[i].clone());
        }
        tr_data.push(col);
    }

    tr_data
}

#[test]
fn test_transpose() {
    let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];

    let actual = transpose(input);

    assert_eq!(expected, actual);
}

fn vec_to_string(input: Vec<char>) -> String {
    let mut result = String::new();
    for c in input {
        result.push(c);
    }
    result
}

fn format_numbers(nums: &Vec<usize>) -> String {
    let mut result = String::new();
    for n in nums.iter() {
        result.push_str(&format!("{} ", n));
    }
    
    result
}
fn gc_content(input: &str) -> f64 {
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

fn consensus_string(data: ProfileMatrix) -> String {
    unimplemented!()
}

fn hamming_distance(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .fold(0, |count, (a, b)|
              if a != b { count + 1 }
              else { count })
}

fn main() {
    let data: Vec<String> = parse_fasta("testdata.txt").values().map(|l| l.clone()).collect();
    let pm = ProfileMatrix::new(data);
    println!("{:?}", pm);
}
