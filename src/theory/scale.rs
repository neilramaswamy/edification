use crate::theory::interval::*;
use lazy_static::lazy_static;

pub struct Scale {
    pub ascending: Vec<Interval>,

    // If None, the descending notes are the same as the ascending notes.
    // If Some, that indicates that the descending notes differ from the ascending notes.
    pub descending: Option<Vec<Interval>>,
}

lazy_static! {
    pub static ref CHROMATIC: Scale = Scale {
        ascending: vec![
            PERF1, MIN2, MAJ2, MIN3, MAJ3, PERF4, TRITONE5, PERF5, MIN6, MAJ6, MIN7, MAJ7,
        ],
        descending: None,
    };

    pub static ref MIXO: Scale = Scale {
        ascending: vec![PERF1, MAJ2, MAJ3, PERF4, PERF5, MAJ6, MIN7],
        descending: None,
    };

    pub static ref MIXO_FLAT9: Scale = Scale {
        ascending: vec![PERF1, MIN2, MAJ3, PERF4, PERF5, MAJ6, MIN7],
        descending: None,
    };

    pub static ref MIXO_FLAT13: Scale = Scale {
        ascending: vec![PERF1, MAJ2, MAJ3, PERF4, PERF5, MIN6, MIN7],
        descending: None,
    };

    // "The LAST thing I'm thinking about on G ALT is G! That shit's Ab melodic minor *stomp*!"
    // -- Ed Tomassi
    pub static ref ALTERED: Scale = Scale {
        ascending: vec![PERF1, MIN2, AUG2, MAJ3, TRITONE5, MIN6, MIN7],
        descending: None,
    };
}
