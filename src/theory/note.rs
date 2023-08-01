use crate::theory::interval::Interval;

use num_derive::FromPrimitive;

use super::scale::Scale;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NoteAccidental {
    None,
    Flat,
    Sharp,
    DoubleFlat,
    DoubleSharp,
}

impl NoteAccidental {
    fn semitone_offset(&self) -> i8 {
        match self {
            NoteAccidental::None => 0,
            NoteAccidental::Flat => -1,
            NoteAccidental::Sharp => 1,
            NoteAccidental::DoubleFlat => -2,
            NoteAccidental::DoubleSharp => 2,
        }
    }
}

impl ToString for NoteAccidental {
    fn to_string(&self) -> String {
        match self {
            NoteAccidental::None => "".to_string(),
            NoteAccidental::Flat => "b".to_string(),
            NoteAccidental::Sharp => "#".to_string(),
            NoteAccidental::DoubleFlat => "bb".to_string(),
            NoteAccidental::DoubleSharp => "##".to_string(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Note {
    // The letter of the note, like A/B/C
    letter: NoteLetter,

    // The accidental, if any.
    accidental: NoteAccidental,

    // The octave of the Note. The Notes corresponding to A4 and A5 are all the same, except for
    // this field.
    octave: i8,
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.semitone_value().cmp(&other.semitone_value());
    }
}

// NoteLetter represents one of the 7 note letters.
//
// Arithmetic on NoteLetters is necessary to spell notes derived from intervals correctly. For
// example, a minor third applied to Db is Fb, not E; the way you get the F in Fb is by going "up"
// three note letters from Db. Critically, you can't use semitones for this task. For this reason,
// we derive FromPrimitive to make arithmetic easier.
#[derive(FromPrimitive, Copy, Clone, PartialEq, Eq, Debug)]
pub enum NoteLetter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl NoteLetter {
    fn semitone_offset(&self) -> i8 {
        match self {
            NoteLetter::C => 0,
            NoteLetter::D => 2,
            NoteLetter::E => 4,
            NoteLetter::F => 5,
            NoteLetter::G => 7,
            NoteLetter::A => 9,
            NoteLetter::B => 11,
        }
    }

    // This shouldn't change unless we start supporting more than 12-tone equal temperament, but
    // if that happens, you should delete this project and start over.
    const NUM_NOTE_LETTERS: i8 = 7;

    // Advances the given letter by `number` note letters. Returns the new NoteLetter, as well as
    // how many octaves the note went up by.
    fn advance_by(&self, number: i8) -> (Self, i8) {
        let new_letter_num = (*self as i8) + number;
        let new_octaves = new_letter_num / NoteLetter::NUM_NOTE_LETTERS;

        match num::FromPrimitive::from_i8(new_letter_num % NoteLetter::NUM_NOTE_LETTERS) {
            Some(note_letter) => (note_letter, new_octaves),
            None => panic!("Always should be able to advance a NoteLetter into another"),
        }
    }
}

impl ToString for NoteLetter {
    fn to_string(&self) -> String {
        match self {
            NoteLetter::A => "A".to_string(),
            NoteLetter::B => "B".to_string(),
            NoteLetter::C => "C".to_string(),
            NoteLetter::D => "D".to_string(),
            NoteLetter::E => "E".to_string(),
            NoteLetter::F => "F".to_string(),
            NoteLetter::G => "G".to_string(),
        }
    }
}

impl Note {
    // Create a Note from a letter and accidental, with a default octave.
    pub fn new(letter: NoteLetter, accidental: NoteAccidental) -> Note {
        Note {
            letter,
            accidental,
            octave: 4,
        }
    }

    pub fn ascending_scale(&self, scale: &Scale) -> Vec<Note> {
        let mut scale_notes = Vec::<Note>::with_capacity(scale.ascending.len());

        for interval in scale.ascending.iter() {
            let scale_note = self.apply_interval(interval);
            scale_notes.push(scale_note)
        }

        scale_notes
    }

    fn semitone_value(&self) -> i8 {
        let octave_offset = 12 * self.octave;
        return octave_offset + self.letter.semitone_offset() + self.accidental.semitone_offset();
    }

    // ApplyInterval moves self by the given interval, and returns the resulting note.
    pub fn apply_interval(&self, interval: &Interval) -> Self {
        let our_letter = self.letter;

        // The NoteLetter corresponding to the returned note is interval.number letters away from
        // our_letter.
        let (new_note_letter, new_octaves) = our_letter.advance_by(interval.number);

        let new_note = &mut Note {
            letter: new_note_letter,
            accidental: NoteAccidental::None,
            octave: self.octave + new_octaves,
        };

        let distance = interval.semitones - new_note.get_semitone_distance(self);
        match distance {
            -2 => new_note.accidental = NoteAccidental::DoubleFlat,
            -1 => new_note.accidental = NoteAccidental::Flat,
            0 => (),
            1 => new_note.accidental = NoteAccidental::Sharp,
            2 => new_note.accidental = NoteAccidental::DoubleSharp,
            i => panic!(
                "All new notes should be within a double flat/sharp of their intended interval, received {i}"
            ),
        }

        *new_note
    }

    fn get_semitone_distance(&self, other: &Self) -> i8 {
        let our_value = dbg!(self.semitone_value());
        let other_value = dbg!(other.semitone_value());
        return our_value - other_value;
    }
}

impl ToString for Note {
    fn to_string(&self) -> String {
        let mut note_str = String::with_capacity(5);

        note_str += &self.letter.to_string();
        note_str += &self.accidental.to_string();
        note_str += &self.octave.to_string();

        note_str
    }
}

#[cfg(test)]
mod tests {
    use crate::theory::{
        interval::{MAJ2, MAJ3, MIN3, MIN6, MIN7, OCT1, PERF1, PERF4, PERF5},
        scale::MIXO_FLAT13,
    };

    use super::*;

    // Test that we can create notes, stringify them, and apply intervals correctly, and
    // generate a scale.

    #[test]
    fn test_notes_stringify() {
        let c4 = Note::new(NoteLetter::C, NoteAccidental::None);
        assert_eq!(c4.to_string(), "C4");

        let fbb = Note::new(NoteLetter::F, NoteAccidental::DoubleFlat);
        assert_eq!(fbb.to_string(), "Fbb4")
    }

    #[test]
    fn test_intervals() {
        let db = Note::new(NoteLetter::D, NoteAccidental::Flat);
        assert_eq!(db.apply_interval(&MIN3).to_string(), "Fb4");

        let cs = Note::new(NoteLetter::C, NoteAccidental::Sharp);
        assert_eq!(cs.apply_interval(&MIN3).to_string(), "E4");

        let f = Note::new(NoteLetter::F, NoteAccidental::None);

        // Use a scale to test: this is Mixob13
        assert_eq!(f.apply_interval(&PERF1).to_string(), "F4");
        assert_eq!(f.apply_interval(&MAJ2).to_string(), "G4");
        assert_eq!(f.apply_interval(&MAJ3).to_string(), "A4");
        assert_eq!(f.apply_interval(&PERF4).to_string(), "Bb4");
        assert_eq!(f.apply_interval(&PERF5).to_string(), "C5");
        assert_eq!(f.apply_interval(&MIN6).to_string(), "Db5");
        assert_eq!(f.apply_interval(&MIN7).to_string(), "Eb5");
        assert_eq!(f.apply_interval(&OCT1).to_string(), "F5");
    }
}
