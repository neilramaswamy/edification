use std::str::FromStr;

use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::Document;

use crate::theory::{
    note::{KeyColor, Note, NoteParseError},
    scale::CHROMATIC,
};

#[derive(Debug)]
struct HighlightedNote {
    note: Note,
    color: HighlightColor,
}

impl HighlightedNote {
    fn to_hex(&self) -> String {
        match &self.color {
            HighlightColor::Red => "#e74c3c".to_string(),
            HighlightColor::Green => "#2ecc71".to_string(),
            HighlightColor::Custom(hex) => hex.to_string(),
        }
    }
}

pub struct Piano {
    // A vector of all the higlighted notes on this piano.
    highlighted: Vec<HighlightedNote>,

    // Width of the piano, in pixels
    width: f32,

    // Height of the piano, in pixels. This is derivable from the width:
    // height = width * 0.877
    height: f32,

    // The number of complete octaves to render, starting at C4
    num_octaves: i8,

    // The amount of horizontal/vertical padding on either side of the piano
    padding_x: f32,
    padding_y: f32,
}

#[derive(Debug)]
pub enum HighlightColor {
    Red,
    Green,
    Custom(String),
}

#[derive(Debug)]
pub enum PianoError {
    InvalidNoteString,
}

impl Piano {
    pub fn new() -> Self {
        let num_octaves: i8 = 2;

        Piano {
            highlighted: vec![],
            width: 256.0,
            height: 256.0 * (0.877 / num_octaves as f32),
            padding_x: 10.0,
            padding_y: 20.0,
            num_octaves,
        }
    }

    // Highlights a single note on the piano instance.
    pub fn highlight_note(&mut self, note: &str, color: HighlightColor) -> Result<(), PianoError> {
        let note = Note::from_str(note).or_else(|err| match err {
            NoteParseError::InvalidFormat => Err(PianoError::InvalidNoteString),
        })?;
        self.highlighted.push(HighlightedNote { note, color });

        Ok(())
    }

    // Renders the piano to the given filepath, returning a Result of the operation.
    pub fn save(&self, filepath: &str) {
        let document = self.render_piano();

        svg::save(filepath, &document).unwrap();
    }

    fn render_piano(&self) -> Document {
        let true_height = self.width + (2.0 * self.padding_y) as f32;
        let true_width = self.width + (2.0 * self.padding_x) as f32;

        let mut document = Document::new().set("viewBox", (0, 0, true_width, true_height));

        // TODO: Support starting from an arbitrary note. For now, 2 octaves.
        let mut notes = Note::from_str("C4").unwrap().ascending_scale(&CHROMATIC);
        notes.append(&mut Note::from_str("C5").unwrap().ascending_scale(&CHROMATIC));

        // To render the keyboard, we maintain a left_offset, which is the left-most point of the
        // last white key that we rendered.
        //
        // If we're _now_ going to render a white key, we render it from the offset and advance it.
        // If we're now going to render a black key, we render it from that offset (adjusted a
        // bit left, since black keys are offset left), and keep the white key where it is.
        //
        // If this algorithm seems arbitrary, it's because it is. Just convince yourself it works
        // by running an example.
        let mut left_offset: f32 = self.padding_x;

        // We keep them in separate groups because SVG z-index is based on ordering in the XML.
        // When inserting these into the SVG, we'll insert the black group above the white group
        // so that the black keys can actually be seen.
        let mut black_notes = Group::new();
        let mut white_notes = Group::new();

        for note in notes {
            let md = self.get_note_render_metadata(&note);

            let key_data = Data::new()
                .move_to((left_offset + md.x_offset, self.padding_y))
                .line_by((0, md.height))
                .line_by((md.width, 0))
                .line_by((0, -md.height))
                .close();

            let path = Path::new()
                .set("fill", md.visual_spec)
                .set("stroke", "black")
                .set("d", key_data);

            // Finally, adjust the left offset
            match note.key_color() {
                KeyColor::White => {
                    white_notes = white_notes.add(path);
                    left_offset += md.width;
                }
                KeyColor::Black => {
                    black_notes = black_notes.add(path);
                }
            }
        }

        document = document.add(white_notes).add(black_notes);
        document
    }

    fn get_note_render_metadata(&self, note: &Note) -> RenderedKeyMetadata {
        // All of these constants are derived from here:
        // https://upload.wikimedia.org/wikipedia/commons/4/48/Pianoteilung.svg

        // TODO: When we support any number of keys, we'll have to update this octave-specific
        // logic.
        let width = match note.key_color() {
            KeyColor::White => self.width / (7.0 * self.num_octaves as f32),
            KeyColor::Black => self.width / (12.0 * self.num_octaves as f32),
        };

        let height = match note.key_color() {
            KeyColor::White => self.height,
            KeyColor::Black => self.height * 0.689,
        };

        // TODO(neil): We should have a notion of enharmonic equality somewhere
        let highlighted_note = self
            .highlighted
            .iter()
            .find(|x| x.note.inter_octave_semitone_value() == note.inter_octave_semitone_value());

        let visual_spec = match note.key_color() {
            KeyColor::White => highlighted_note.map_or("white".to_string(), |v| v.to_hex()),
            KeyColor::Black => highlighted_note.map_or("black".to_string(), |v| v.to_hex()),
        };

        let x_offset = match note.key_color() {
            KeyColor::White => 0.0,
            KeyColor::Black => {
                // Eventually, we should do this off of note letter equivalence classes and not
                // the intra octave semitones, because this is exceptionally jank.
                let octave_width = self.width / self.num_octaves as f32;

                match note.intra_octave_semitone_value() {
                    // Db
                    1 => -octave_width * 0.0518,
                    // Eb
                    3 => -octave_width * 0.0251,
                    // Gb
                    6 => -octave_width * 0.0584,
                    // Ab
                    8 => -octave_width * 0.0384,
                    // Bb
                    10 => -octave_width * 0.0184,
                    _ => panic!("Black key found with invalid intra octave semitone value"),
                }
            }
        };

        RenderedKeyMetadata {
            width,
            height,
            x_offset,
            visual_spec,
        }
    }
}

struct RenderedKeyMetadata {
    width: f32,
    height: f32,

    // Used only for black keys to know how much to the left we should render them,
    // relative to the current white key.
    x_offset: f32,

    visual_spec: String,
}
