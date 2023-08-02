use std::str::FromStr;

use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::Document;

use crate::theory::{
    note::{KeyColor, Note, NoteParseError},
    scale::CHROMATIC,
};

pub struct Piano {
    // A vector of all the higlighted notes on this piano.
    highlighted: Vec<Note>,

    // Width of the piano, in pixels
    width: f32,

    // Height of the piano, in pixels. This is derivable from the width:
    // height = width * 0.877
    height: f32,
}

#[derive(Debug)]
pub enum PianoError {
    InvalidNoteString,
}

impl Piano {
    pub fn new() -> Self {
        Piano {
            highlighted: vec![],
            width: 1024.0,
            height: 1024.0 * 0.877,
        }
    }

    // Highlights a single note on the piano instance.
    pub fn highlight_note(&mut self, note: &str) -> Result<(), PianoError> {
        let note = Note::from_str(note).or_else(|err| match err {
            NoteParseError::InvalidFormat => Err(PianoError::InvalidNoteString),
        })?;
        self.highlighted.push(note);

        Ok(())
    }

    // Renders the piano to the given filepath, returning a Result of the operation.
    pub fn save(&self) {
        let document = self.render_piano();

        svg::save("piano.svg", &document).unwrap();
    }

    fn render_piano(&self) -> Document {
        let mut document =
            Document::new().set("viewBox", (0, 0, self.width + 10.0, self.height + 10.0));

        // TODO: Support starting from an arbitrary note. For now, 2 octaves.
        let notes = Note::from_str("C4").unwrap().ascending_scale(&CHROMATIC);
        // notes.append(&mut Note::from_str("C5").unwrap().ascending_scale(&CHROMATIC));

        // To render the keyboard, we maintain a left_offset, which is the left-most point of the
        // last white key that we rendered.
        //
        // If we're _now_ going to render a white key, we render it from the offset and advance it.
        // If we're now going to render a black key, we render it from that offset (adjusted a
        // bit left, since black keys are offset left), and keep the white key where it is.
        //
        // If this algorithm seems arbitrary, it's because it is. Just convince yourself it works
        // by running an example.
        let mut left_offset: f32 = 0.0;

        // We keep them in separate groups because SVG z-index is based on ordering in the XML.
        // When inserting these into the SVG, we'll insert the black group above the white group
        // so that the black keys can actually be seen.
        let mut black_notes = Group::new();
        let mut white_notes = Group::new();

        for note in notes {
            let md = self.get_note_render_metadata(&note);

            let key_data = Data::new()
                .move_to((left_offset + md.x_offset, 0))
                .line_by((0, md.height))
                .line_by((md.width, 0))
                .line_by((0, -md.height))
                .close();

            let path = Path::new()
                .set("fill", md.visual_spec)
                .set("stroke", "black")
                .set("stroke-width", 3)
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
        let num_octaves = 1;

        let width = match note.key_color() {
            KeyColor::White => self.width / (7.0 * num_octaves as f32),
            KeyColor::Black => self.width / (12.0 * num_octaves as f32),
        };

        let height = match note.key_color() {
            KeyColor::White => self.height,
            KeyColor::Black => self.height * 0.689,
        };

        // TODO(neil): We should have a notion of enharmonic equality somewhere
        let is_highlighted = self
            .highlighted
            .iter()
            .find(|x| x.inter_octave_semitone_value() == note.inter_octave_semitone_value())
            .is_some();

        let visual_spec = match note.key_color() {
            KeyColor::White => {
                if is_highlighted {
                    "#DD0369".to_string()
                } else {
                    "white".to_string()
                }
            }
            KeyColor::Black => {
                if is_highlighted {
                    "#C9035F".to_string()
                } else {
                    "black".to_string()
                }
            }
        };

        let x_offset = match note.key_color() {
            KeyColor::White => 0.0,
            KeyColor::Black => {
                // Eventually, we should do this off of note letter equivalence classes and not
                // the intra octave semitones, because this is exceptionally jank.
                match note.intra_octave_semitone_value() {
                    // Db
                    1 => -self.width * 0.0518,
                    // Eb
                    3 => -self.width * 0.0251,
                    // Gb
                    6 => -self.width * 0.0584,
                    // Ab
                    8 => -self.width * 0.0384,
                    // Bb
                    10 => -self.width * 0.0184,
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
