#[derive(Debug)]
pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

// Interval is only ascending, for now.
#[derive(Debug)]
pub struct Interval {
    pub quality: IntervalQuality,

    // The 0-indexed number of named notes in the Interval.
    // A minor 2nd is 2, an augmented 7th is 7, etc.
    pub number: i8,
    pub semitones: i8,
}

pub const PERF1: Interval = Interval {
    quality: IntervalQuality::Perfect,
    number: 0,
    semitones: 0,
};
pub const MIN2: Interval = Interval {
    quality: IntervalQuality::Minor,
    number: 1,
    semitones: 1,
};
pub const MAJ2: Interval = Interval {
    quality: IntervalQuality::Major,
    number: 1,
    semitones: 2,
};

pub const AUG2: Interval = Interval {
    quality: IntervalQuality::Augmented,
    number: 1,
    semitones: 3,
};
pub const MIN3: Interval = Interval {
    quality: IntervalQuality::Minor,
    number: 2,
    semitones: 3,
};
pub const MAJ3: Interval = Interval {
    quality: IntervalQuality::Major,
    number: 2,
    semitones: 4,
};
pub const PERF4: Interval = Interval {
    quality: IntervalQuality::Perfect,
    number: 3,
    semitones: 5,
};
pub const TRITONE4: Interval = Interval {
    quality: IntervalQuality::Augmented,
    number: 3,
    semitones: 6,
};
pub const TRITONE5: Interval = Interval {
    quality: IntervalQuality::Diminished,
    number: 4,
    semitones: 6,
};
pub const PERF5: Interval = Interval {
    quality: IntervalQuality::Perfect,
    number: 4,
    semitones: 7,
};
pub const MIN6: Interval = Interval {
    quality: IntervalQuality::Minor,
    number: 5,
    semitones: 8,
};
pub const MAJ6: Interval = Interval {
    quality: IntervalQuality::Major,
    number: 5,
    semitones: 9,
};
pub const MIN7: Interval = Interval {
    quality: IntervalQuality::Minor,
    number: 6,
    semitones: 10,
};
pub const MAJ7: Interval = Interval {
    quality: IntervalQuality::Major,
    number: 6,
    semitones: 11,
};
pub const OCT1: Interval = Interval {
    quality: IntervalQuality::Perfect,
    number: 7,
    semitones: 12,
};

// Tensions
//
// These are kept in the same octave (i.e. `semitone` isn't >= 12) because in Jazz, a T9 doesn't
// mean "something that's a major 9th away".

pub const T9: Interval = Interval {
    quality: IntervalQuality::Major,
    number: 1,
    semitones: 2,
};

pub const TFLAT9: Interval = Interval {
    quality: IntervalQuality::Minor,
    number: 1,
    semitones: 1,
};

pub const T11: Interval = Interval {
    quality: IntervalQuality::Perfect,
    number: 3,
    semitones: 5,
};

pub const TSHARP11: Interval = Interval {
    quality: IntervalQuality::Augmented,
    number: 3,
    semitones: 6,
};

pub const T13: Interval = Interval {
    quality: IntervalQuality::Major,
    number: 5,
    semitones: 9,
};

pub const TFLAT13: Interval = Interval {
    quality: IntervalQuality::Minor,
    number: 5,
    semitones: 8,
};
