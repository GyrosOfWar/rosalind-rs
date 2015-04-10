mod profile_matrix;
mod genome;

fn main() {
    let motif_str = "N{P}[ST]{P}";

    let mut p = genome::ProteinMotif::new(motif_str);
    let motif = p.parse();
    println!("{:?}", motif);
}
