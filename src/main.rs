extern crate hyper;

use std::collections::HashMap;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env::args;
use hyper::Client;
use protein_motif::ProteinMotif;

mod profile_matrix;
mod genome;
mod protein_motif;
mod strext;

fn fetch_from_uniprot(protein_id: &str) -> HashMap<String, String> {
    let mut client = Client::new();
    let url: &str = &format!("http://www.uniprot.org/uniprot/{}.fasta", protein_id);
    let mut response = client.get(url).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body);

    genome::parse_fasta(&body)
}

fn protein_motif() {
    let motif_str = "N{P}[ST]{P}";
    let mut p = ProteinMotif::new(motif_str);
    p.parse();

    let file_name = args().nth(1).unwrap();
    let file = BufReader::new(File::open(Path::new(&file_name)).unwrap());

    for line in file.lines() {
        let id = line.unwrap();
        let data = fetch_from_uniprot(&id);
        let values: Vec<_> = data.values().collect();
        let indices = p.find_motif(&values[0]);
        if indices.len() > 0 {
            println!("{}", id);
            for i in indices {
                print!("{} ", i);
            }
            println!("");
        }
    }
}

fn main() {
    let mut file = File::open(Path::new("rosalind_sseq.txt")).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data);
    let mut fasta = genome::parse_fasta(&data);

    let haystack = &fasta["Rosalind_2269"];
    let needle = &fasta["Rosalind_7249"];

    let subsq = genome::find_subsequence(haystack, needle);
    let subsq_str = subsq.iter().fold(String::new(), |s, i| s + &i.to_string() + " ");
    println!("{}", subsq_str);
}
