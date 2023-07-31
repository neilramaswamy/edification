use crate::theory::interval::Interval;

use num_derive::FromPrimitive;

use super::scale::Scale;

enum NoteAccidental {
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
            Flat => -1,
            Sharp => 1,
            DoubleFlat => -2,
            DoubleSharp => 2,
        }
    }
}

impl ToString for NoteAccidental {
    fn to_string(&self) -> String {
        match self {
            NoteAccidental::None => "".to_string(),
            Flat => "b".to_string(),
            Sharp => "#".to_string(),
            DoubleFlat => "bb".to_string(),
            DoubleSharp => "##".to_string(),
        }
    }
}

pub struct Note {
    // The letter of the note, like A/B/C
    letter: NoteLetter,

    // The accidental, if any.
    accidental: NoteAccidental,

    // The octave of the Note. The Notes corresponding to A4 and A5 are all the same, except for
    // this field.
    octave: i8,
}

// NoteLetter represents one of the 7 note letters.
//
// Arithmetic on NoteLetters is necessary to spell notes derived from intervals correctly. For
// example, a minor third applied to Db is Fb, not E; the way you get the F in Fb is by going "up"
// three note letters from Db. Critically, you can't use semitones for this task. For this reason,
// we derive FromPrimitive to make arithmetic easier.
#[derive(FromPrimitive, Copy, Clone)]
enum NoteLetter {
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
            C => 0,
            D => 2,
            E => 4,
            F => 5,
            G => 7,
            A => 9,
            B => 11,
            _ => panic!("Received incorrect NoteLetter"),
        }
    }

    // This shouldn't change unless we start supporting more than 12-tone equal temperament, but
    // if that happens, you should delete this project and start over.
    const NUM_NOTE_LETTERS: i8 = 7;

    // Advances the given letter by `number` note letters. Returns the new NoteLetter, as well as
    // how many octaves the note went up by.
    fn advance_by(&self, number: i8) -> (Self, i8) {
        let new_letter_num = (*self as i8) + number;
        let new_octaves = new_letter_num / NUM_NOTE_LETTERS;

        match num::FromPrimitive::from_i8(new_letter_num % NUM_NOTE_LETTERS) {
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

// The size of NoteLetter. This should never change as long as we only support 12-tone equal
// temperament.
const NUM_NOTE_LETTERS: i8 = 7;

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
        let scale_notes = &mut Vec::<Note>::with_capacity(scale.ascending.len());

        let mut i = 0;
        for interval in scale.ascending.iter() {
            let scale_note = self.apply_interval(interval);
            scale_notes[i] = scale_note;
        }

        *scale_notes
    }

    fn semitone_value(&self) -> i8 {
        let octave_offset = 7 * self.octave;
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
            _ => panic!(
                "All new notes should be within a double flat/sharp of their intended interval"
            ),
        }

        *new_note
    }

    fn get_semitone_distance(&self, other: &Self) -> i8 {
        let our_value = self.semitone_value();
        let other_value = self.semitone_value();
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
