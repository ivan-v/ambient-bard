use phf::phf_map;

use crate::chord::STARTING_PITCH;

pub static MODES: phf::Map<&'static str, &'static [i32]> = phf_map! {
    "Ionian" => &[0, 2, 4, 5, 7, 9, 11],
    "Mixolydian" => &[0, 2, 4, 5, 7, 9, 10],
    "Lydian" => &[0, 2, 4, 6, 7, 9, 11],
    "Dorian" => &[0, 2, 3, 5, 7, 9, 10],
    "Phrygian" => &[0, 1, 3, 5, 7, 8, 10],
    "Aeolian" => &[0, 2, 3, 5, 7, 8, 10],
};

pub struct Applied_Key<'a> {
    pub name: String,
    pub root: &'a str,
    pub tones: &'a [i32],
    pub aps: Vec<i32>,
}

impl<'a> Applied_Key<'a> {
    pub fn new(root: &'a str, mode: &'a str) -> Self {
        let tones = MODES[mode];
        let mut aps = Vec::new();
        for octave in -4..4 {
            for pitch in tones.iter() {
                let ap = pitch + octave * 12 + STARTING_PITCH[root];
                aps.push(ap);
            }
        }
        Self {
            name: format!("{} {}", root, mode),
            root,
            tones,
            aps,
        }
    }
}