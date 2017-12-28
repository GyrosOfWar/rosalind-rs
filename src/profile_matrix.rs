#![allow(dead_code)]

use std::fmt;
use std::ops;
use genome::count_nucleotides;

#[derive(PartialEq, Eq)]
pub struct ProfileMatrix<'a> {
    data: [Vec<usize>; 4],
    source: &'a Vec<String>,
    length: usize,
}

impl<'a> ProfileMatrix<'a> {
    pub fn new(data: &'a Vec<String>) -> ProfileMatrix<'a> {
        let size = data[0].len();
        let mut result = [
            Vec::with_capacity(size),
            Vec::with_capacity(size),
            Vec::with_capacity(size),
            Vec::with_capacity(size),
        ];

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
            length: size,
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
    A = 0,
    C = 1,
    G = 2,
    T = 3,
}
use self::Nucleotide::{A, C, G, T};

impl<'a> ops::Index<Nucleotide> for ProfileMatrix<'a> {
    type Output = Vec<usize>;
    fn index(&self, idx: Nucleotide) -> &Vec<usize> {
        match idx {
            A => &self.data[0],
            C => &self.data[1],
            G => &self.data[2],
            T => &self.data[3],
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
