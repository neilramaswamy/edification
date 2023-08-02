pub mod drawing;
pub mod theory;

fn main() {
    println!("Because Rust is Turing complete and all my modules compile, that means my code is correct and will halt, via the Halting Theorem.");

    let mut piano = drawing::piano::Piano::new();

    piano.highlight_note("Fbb").unwrap();
    piano.highlight_note("E4").unwrap();
    piano.highlight_note("A##").unwrap();

    piano.save();
}
