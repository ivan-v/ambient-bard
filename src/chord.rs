use phf::{phf_map};

use crate::applied_key::Applied_Key;

pub static CHORD_TYPE_TO_PITCHES: phf::Map<&'static str, &'static [i32]> = phf_map! {
    "M" => &[0, 4, 7],
    "m" => &[0, 3, 7],
    "suss" => &[0, 5, 7],
    "dim" => &[0, 3, 6],
    "aug" => &[0, 4, 8],
    "M7" => &[0, 4, 7, 11],
    "m7" => &[0, 3, 7, 10],
    "7" => &[0, 4, 7, 10],
    "m7b5" => &[0, 3, 6, 10],
    "7b5" => &[0, 4, 6, 10],
    "6" => &[0, 4, 7, 9],
    "m6" => &[0, 3, 7, 9],
    "7#5" => &[0, 4, 8, 10],
    "m+9" => &[0, 3, 7, 14],
    "dim7" => &[0, 3, 6, 9],
    "halfdim7" => &[0, 3, 6, 10],
    "m9" => &[0, 3, 7, 10, 14],
    "Dom.min9" => &[0, 4, 7, 10, 11],
    "9" => &[0, 4, 7, 10, 14],
    "b9" => &[0, 4, 7, 10, 13],
    "M9" => &[0, 4, 7, 11, 14],
    "11" => &[0, 7, 10, 14, 17],
    "7#9" => &[0, 4, 7, 10, 15],
    "7#11" => &[0, 4, 7, 10, 18],
    "m11" => &[0, 3, 7, 10, 14, 17],
    "M7#11" => &[0, 4, 7, 11, 14, 18],
    "13" => &[0, 4, 7, 10, 14, 21],
    "M13" => &[0, 4, 7, 11, 14, 21],
    "m13" => &[0, 3, 7, 10, 14, 17, 21],
};

pub static STARTING_PITCH: phf::Map<&'static str, i32> = phf_map! {
    "C" => 60,
    "Cs" => 61,
    "Db" => 61,
    "D" => 62,
    "Ds" => 63,
    "Eb" => 63,
    "E" => 64,
    "Fb" => 64,
    "Es" => 65,
    "F" => 65,
    "Fs" => 66,
    "Gb" => 66,
    "G" => 67,
    "Gs" => 68,
    "Ab" => 68,
    "A" => 69,
    "As" => 70,
    "Bb" => 70,
    "B" => 71,
    "Cb" => 71,
    "Bs" => 60,
};

fn roman_numeral_to_note(applied_key_root: String, applied_key_tones: &[i32], roman_numeral: &str) -> i32 {
    let mut modifier = 0;
    let mut roman_numeral = roman_numeral.to_string();

    if roman_numeral.starts_with("b") {
        modifier = -1;
        roman_numeral = roman_numeral[1..].to_string();
    } else if roman_numeral.starts_with("#") {
        modifier = 1;
        roman_numeral = roman_numeral[1..].to_string();
    }

    let r_n = roman_numeral.to_lowercase();
    match r_n.as_str() {
        "i" => applied_key_tones[0] + modifier + STARTING_PITCH[&applied_key_root],
        "ii" => applied_key_tones[1] + modifier + STARTING_PITCH[&applied_key_root],
        "iii" => applied_key_tones[2] + modifier + STARTING_PITCH[&applied_key_root],
        "iv" => applied_key_tones[3] + modifier + STARTING_PITCH[&applied_key_root],
        "v" => applied_key_tones[4] + modifier + STARTING_PITCH[&applied_key_root],
        "vi" => applied_key_tones[5] + modifier + STARTING_PITCH[&applied_key_root],
        "vii" => applied_key_tones[6] + modifier + STARTING_PITCH[&applied_key_root],
        _ => panic!("Numeral not found in applied_key: {}", roman_numeral),
    }
}

pub struct Chord {
    pub type_: String,
    pub root: String,
    pub operating_bit: String,
    pub aps: Vec<Vec<i32>>,
    pub pitches: Vec<i32>,
}

impl Chord {
    pub fn new(root: String, type_: Option<String>, operating_bit: Option<String>, pitches: Option<Vec<i32>>) -> Self {
        let type_ = type_.unwrap_or(if root.chars().next().unwrap().is_lowercase() { "m".to_owned() } else { "M".to_owned() });
        let operating_bit = operating_bit.unwrap_or_default();
        let aps:Vec<Vec<i32>> = if !operating_bit.is_empty() {
            let undertone = get_undertone(type_.contains("m"), operating_bit.clone());
            (0..8).map(|octave| {
                CHORD_TYPE_TO_PITCHES[&type_].iter().map(|pitch| pitch + octave * 12 + STARTING_PITCH[&root]).chain(std::iter::once(undertone + octave * 12 + STARTING_PITCH[&root])).collect()
            }).collect()
        } else {
            (0..8).map(|octave| {
                let foo = &root;
                CHORD_TYPE_TO_PITCHES[&type_].iter().map(|pitch| pitch + octave * 12 + STARTING_PITCH[&root]).collect()
            }).collect()
        };
        let pitches = pitches.unwrap_or_else(|| aps[4].to_vec());
        Self {
            root,
            type_,
            operating_bit,
            aps,
            pitches,
        }
    }
}

pub fn roman_progression_to_chords(applied_key: Applied_Key, roman_progression: &Vec<Vec<&str>>) -> Vec<Chord> {
    let mut result = Vec::new();
    for chord in roman_progression {
        let note = roman_numeral_to_note(applied_key.root.to_string(), applied_key.tones, chord[0]);
        let root_note = STARTING_PITCH.into_iter().find_map(|(key, &val)| if val == note { Some(key) } else { None }).unwrap();
        let chord_type_split: Vec<&str> = chord[1].split('/').collect();
        let operating_bit = if chord_type_split.len() < 2 { "" } else { chord_type_split[1] };
        let chord_type = chord_type_split[0];
        result.push(Chord::new(root_note.to_string(), Some(chord_type.to_string()), Some(operating_bit.to_string()),None));
    }
    result
}

fn get_undertone(is_minor: bool, operating_bit: String) -> i32 {
    let tones: [i32; 7] = if is_minor {
        [0, 2, 3, 5, 7, 8, 10]
    } else {
        [0, 2, 4, 5, 7, 9, 11]
    };

    if operating_bit.len() > 1 {
        let modifier = match &operating_bit[..1] {
            "b" => -1,
            "#" => 1,
            _ => 0,
        };
        let index = operating_bit[1..].parse::<usize>().unwrap() - 1;
        return tones[index] + modifier - 12;
    } else {
        let index = operating_bit.parse::<usize>().unwrap() - 1;
        return tones[index] - 12;
    }
}


