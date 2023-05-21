use std::collections::{HashMap, HashSet};

use crate::{motif::{self, Motif}, forms::{FORMS, sync_note_durations}, 
applied_key::AppliedKey, presets::Presets};


pub struct Song<'a> {
    pub presets: Presets,
    pub motifs: HashMap<&'a str, Motif>,
    pub form: &'static [&'static str],
    pub notes: Vec<(i32, f32, f32)>
}

impl Song<'_> {
    pub fn new(form_type: &str, presets: Presets) -> Self {
        let form = FORMS[form_type];
        let unique_parts_dict: HashSet<&'static str> 
            = form.iter().cloned().collect();
        let unique_parts_vec: Vec<&'static str> 
            = unique_parts_dict.into_iter().collect();
        let motifs: HashMap<&str, Motif> = unique_parts_vec.into_iter()
            .map(|part_name| 
                (part_name, motif::Motif::new(presets.clone()))).collect();
        
        let motifs_copy = motifs.clone();
        Self {
            form: form,
            motifs: motifs_copy,
            presets: presets,
            // Match parts (motifs) to form
            notes: sync_note_durations(
                form.into_iter()
                .flat_map(|part| motifs[part].melody.notes.clone())
                .collect(),
                 Some(0.0))
        }
    }
}

pub fn generate_song() -> Vec<(i32, f32, f32)> {
    let applied_key = AppliedKey::new("D", "Ionian");
    let is_major = true;
    let meter = [4.0, 4.0];
    let rhythm_intensity = "7".to_string();
    let max_step_size = 14;
    let span = (58, 88);
    let desired_chord_progression_length = 3;
    let form_type = "Ballade";
    let measure_count = 4;
    let beats = vec![1.0, 0.0, 1.0, 0.0];
    let presets = Presets::new(
        meter, rhythm_intensity, max_step_size, 
        span, applied_key, beats, 
        desired_chord_progression_length, is_major, measure_count);

    let cloned_presets = presets.clone();

    Song::new(form_type, cloned_presets).notes
}