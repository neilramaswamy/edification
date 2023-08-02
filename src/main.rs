use std::{
    fs::{self, create_dir, create_dir_all},
    str::FromStr,
};

use drawing::piano::HighlightColor;
use theory::{interval::TFLAT13, note::Note, scale::CHROMATIC};

pub mod drawing;
pub mod theory;

fn main() {
    let chromatic = Note::from_str("C").unwrap().ascending_scale(&CHROMATIC);

    // Ensure that images/ exists
    create_dir_all("images").unwrap();

    for note in chromatic.iter() {
        // For all of these, let's generate a piano with the Tb13
        let path = format!("images/{note}-Tb13.svg");

        let mut piano = drawing::piano::Piano::new();

        piano
            .highlight_note(&note.to_string(), HighlightColor::Red)
            .unwrap();
        piano
            .highlight_note(
                &note.apply_interval(&TFLAT13).to_string(),
                HighlightColor::Green,
            )
            .unwrap();

        piano.save(&path);
    }
}
