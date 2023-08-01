pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

// Interval is only ascending, for now.
pub struct Interval {
    pub quality: IntervalQuality,
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
