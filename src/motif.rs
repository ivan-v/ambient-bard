use crate::{presets::Presets, chord::Chord, melody::Melody};

#[derive(Clone)]
pub struct Motif {
    pub name: Option<String>,
    pub chords: Vec<Chord>,
    pub melody: Melody,
    pub presets: Presets
}

impl<'a> Motif {
    pub fn new(presets: Presets) -> Self {
        let chords = crate::chord::generate_chord_progression(
            presets.desired_chord_progression_length, presets.is_major, &presets.applied_key);
        let chords_copy = chords.clone();
        
        Self {
            name: std::option::Option::Some("foobar".to_owned()),
            chords: chords,
            melody: Melody::new(chords_copy, presets.clone()),
            presets: presets.clone()
        }
    }
}
