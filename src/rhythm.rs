use std::collections::HashSet;

use phf::phf_map;
use rand::{Rng, seq::SliceRandom};

pub static SPACE_VALUES: phf::Map<&'static str, f32> = phf_map! [
    "wn" => 4.0,
    "hn" => 2.0,
    "qn" => 1.0,
    "en" => 0.5,
    "(3 % 8)" => 1.5,
    "(1 % 3)" => 1.0 / 3.0,
    "dhn" => 3.0,
    "dqn" => 1.5,
    "den" => 0.75,
    "sn" => 0.25,
    "(2 % 3)" => 2.0 / 3.0,
];

pub static METER_TO_SPACE_VALUES: phf::Map<&'static str, f32> = phf_map! ["4" => 1.0, "8" => 0.5];

pub static RHYTHM_PDF_PRESETS: phf::Map<&'static str, phf::Map<&'static str, f32>> = phf_map![
    "0" => phf_map! ["hn" => 0.8, "qn" => 0.2],
    "1" => phf_map! ["hn" => 0.7, "qn" => 0.3, "en" => 0.0],
    "2" => phf_map! ["hn" => 0.6, "qn" => 0.4, "en" => 0.0],
    "3" => phf_map! ["hn" => 0.5, "qn" => 0.5, "en" => 0.0],
    "4" => phf_map! ["hn" => 0.3, "qn" => 0.6, "en" => 0.1],
    "5" => phf_map! ["hn" => 0.2, "qn" => 0.6, "en" => 0.2],
    "6" => phf_map! ["hn" => 0.1, "qn" => 0.6, "en" => 0.3],
    "7" => phf_map! ["hn" => 0.05, "qn" => 0.5, "en" => 0.45],
    "8" => phf_map! ["hn" => 0.05, "qn" => 0.4, "en" => 0.55],
    "9" => phf_map! ["hn" => 0.0, "qn" => 0.3, "en" => 0.7],
    "10" => phf_map! ["hn" => 0.0, "qn" => 0.1, "en" => 0.9]
];

fn check_space(measure_count: i32) -> bool {
    let mut space_for_repeating = false;
    if measure_count > 2 {
        space_for_repeating = true;
    }
    space_for_repeating
}

fn generate_rhythm_measure(space_left: f32, rhythm_pdf: &phf::Map<&'static str, f32>, meter: [f32; 2], given_downbeats: Option<Vec<f32>>) -> Vec<String> {
    let mut measure: Vec<String> = Vec::new();
    let num_downbeats = METER_TO_SPACE_VALUES[&meter[1].to_string()] * meter[0];
    let downbeat_after = METER_TO_SPACE_VALUES[&meter[1].to_string()];
    let downbeats: Vec<f32>;
    if given_downbeats != None {
        downbeats = given_downbeats.unwrap_or_default();
    } else {
        downbeats = (0..num_downbeats as i32).map(|x| (x as f32) / (1.0 / downbeat_after as f32)).collect();
    }
    let pdf: Vec<f32> = rhythm_pdf.values().cloned().collect();
    let pmf: Vec<f32> = pdf.iter().scan(0.0, |acc, x| {
        *acc += x;
        Some(*acc)
    }).collect();

    let mut space_left = space_left;

    while space_left > 0.0 {
        let r: f32 = rand::thread_rng().gen_range(0..100) as f32 / 100.0;
        let p = pmf.iter().find(|&&x| x > r).unwrap();
        let note = rhythm_pdf.keys().nth(pmf.iter().position(|&x| x == *p).unwrap()).unwrap();
        // making sure that the note doesn't (start and end) on non-downbeat
        if downbeats.contains(&(space_left - SPACE_VALUES[note])) && space_left >= SPACE_VALUES[note] {
            if note == &"(3 % 8)" && space_left >= 2.0 {
                space_left -= 2.0;
                measure.push(note.to_string());
                measure.push("en".to_string());
            } else if note == &"(1 % 3)" && space_left >= 1.0 {
                space_left -= 1.0;
                measure.push(note.to_string());
                measure.push(note.to_string());
                measure.push(note.to_string());
            } else if note != &"(1 % 3)" {
                space_left -= SPACE_VALUES[note];
                measure.push(note.to_string());
            }
        }
    }
    return measure;
}

pub fn generate_rhythm(meter: [f32; 2], measure_count: i32, rhythm_pdf: &phf::Map<&'static str, f32>) -> Vec<Vec<String>> {
    let breathing_space = check_space(measure_count);
    let mut rhythm:Vec<Vec<String>> = Vec::new();
    let first_measure = generate_rhythm_measure((meter[0] as f32 / (meter[1] as f32 / 4.0)).ceil() as f32, &rhythm_pdf, meter, None);
    let mut unique_measures = HashSet::new();
    unique_measures.insert(first_measure.clone());
    let mut measures_left = measure_count - 1;
    rhythm.append(&mut vec![first_measure]);
    while measures_left > 0 {
        if breathing_space && rand::thread_rng().gen_range(0..=2) == 0 {
            let mut rng = rand::thread_rng();
            let random_measure = (&unique_measures.iter().map(|v| v.iter().cloned()
                .collect::<Vec<String>>()).collect::<Vec<Vec<String>>>())
                .choose(&mut rng).unwrap().clone();
            rhythm.append(&mut vec![random_measure.clone()]);
        } else {
            let new_measure = generate_rhythm_measure((meter[0] as f32 / (meter[1] as f32 / 4.0)).ceil() as f32, &rhythm_pdf, meter, None);
            if !unique_measures.contains(&new_measure) {
                unique_measures.insert(new_measure.clone());
            }
            rhythm.append(&mut vec![new_measure.clone()]);
        }
        measures_left -= 1;
    }
    rhythm
}