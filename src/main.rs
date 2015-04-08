use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

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
pub struct ProfileMatrix<'a> {
    data: [Vec<usize>; 4],
    source: &'a Vec<String>,
    length: usize
}

impl<'a> ProfileMatrix<'a> {
    pub fn new(data: &'a Vec<String>) -> ProfileMatrix<'a> {
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
            data: result,
            source: data,
            length: size
        }
    }

    pub fn len(&'a self) -> usize {
        self.length
    }

    pub fn consensus_string(&'a self) -> String {
        let mut result = String::new();
        for i in 0..self.len() {
            let mut max_val = 0;
            let mut max_nuc: Option<Nucleotide> = None;
            for x in vec![A, C, G, T] {
                let val = self[x][i];
                if val > max_val {
                    max_nuc = Some(x);
                    max_val = val;
                }
            }
            result.push_str(&format!("{:?}", max_nuc.unwrap()));
        }
        
        result
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Nucleotide {
    A = 0, C = 1, G = 2, T = 3
}
use Nucleotide::{A, C, G, T};

impl<'a> std::ops::Index<Nucleotide> for ProfileMatrix<'a> {
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


impl<'a> fmt::Debug for ProfileMatrix<'a> {
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

fn hamming_distance(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .fold(0, |count, (a, b)|
              if a != b { count + 1 }
              else { count })
}

fn main() {
    let table = "A 71.03711
C 103.00919
D 115.02694
E 129.04259
F 147.06841
G 57.02146
H 137.05891
I 113.08406
K 128.09496
L 113.08406
M 131.04049
N 114.04293
P 97.05276
Q 128.05858
R 156.10111
S 87.03203
T 101.04768
V 99.06841
W 186.07931
Y 163.06333";
    let data: HashMap<char, f64> = table
        .split('\n')
        .map(|s| {
            let split: Vec<_> = s.split(' ').collect();
            (split[0].chars().nth(0).unwrap(), f64::from_str(split[1]).unwrap())
        })
        .collect();
    
    let sum = "SKADYEK".chars().map(|c| data[&c]).fold(0.0, |sum, x| sum + x);
    println!("{}", sum);
}
