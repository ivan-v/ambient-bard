use rand::{Rng, seq::SliceRandom};

use crate::{chord::{Chord, self}, presets::{Presets}, 
rhythm::{RHYTHM_PDF_PRESETS, self, SPACE_VALUES}, applied_key::AppliedKey};

#[derive(Clone)]
pub struct Melody {
    pub notes: Vec<(i32, f32, f32)>,
    pub rhythm: Vec<String>
}

impl<'a> Melody {
    pub fn new(chords: Vec<Chord>, presets: Presets) -> Self {

        let rhythm: Vec<String> = rhythm::generate_rhythm(
            presets.meter, presets.measure_count, 
            &RHYTHM_PDF_PRESETS[&presets.rhythm_intensity]).concat();        
        let rhythm_copy = rhythm.clone();    

        let notes = Self::generate_melody_from_chords(
            rhythm, chords, presets.max_step_size, 
            presets.span, &presets.applied_key);
        Self {
            notes: notes,
            rhythm: rhythm_copy
        }
    }

    pub fn generate_melody_from_chords(
        rhythm: Vec<String>,
        chords: Vec<chord::Chord>,
        max_step_size: i32,
        span: (i32, i32),
        applied_key: &AppliedKey) -> Vec<(i32, f32, f32)> {
        
        let rhythm_copy = rhythm.clone();
        
        let pitches =  Self::generate_pitches_for_melody(
            rhythm, chords, max_step_size, span, &applied_key);
        let solution: Vec<(i32, f32, f32)> 
            = Self::merge_pitches_with_rhythm(&pitches, &rhythm_copy);
        
        solution
    }

    fn generate_chord_times_pdf(
        chords: Vec<Chord>, beats: (Vec<f32>, &'static str))
         -> Vec<Chord> {
        let mut previous_chord_time = 0.0;
        let mut result = Vec::new();
        for chord in chords {
            let mut foo = chord.clone();
            foo.time = Some((previous_chord_time, (beats.0.len() as f32) 
                * SPACE_VALUES[beats.1] + previous_chord_time));
            result.push(foo);
            previous_chord_time += (beats.0.len() as f32) * SPACE_VALUES[beats.1];
        }
        result
    }
    
    fn determine_motion(pitch_1: i32, pitch_2: i32) -> String {
        if (pitch_1 - pitch_2).abs() > 2 {
            "disjunct".to_string()
        } else {
            "conjunct".to_string()
        }
    }

    pub fn generate_pitches_for_melody(
        rhythm: Vec<String>,
        mut chords: Vec<Chord>,
        max_step_size: i32,
        span: (i32, i32),
        applied_key: &AppliedKey,
    ) -> Vec<i32> {
        chords = Self::generate_chord_times_pdf(
            chords, (vec![1.0, 0.0, 1.0, 0.0], "qn"));
        let mut result: Vec<i32>
             = vec![clamp_to_range(chords[0].pitches[0], span)];
        let mut expected_motion = if rand::thread_rng().gen_bool(0.5) {
            "conjunct"
        } else {
            "disjunct"
        };
        let mut current_motion_count = 0;
        let mut motion_limit = 3;
        let mut previous_x_notes: Vec<i32>;
        
        let mut rng = rand::thread_rng();
    
        let mut current_time = 0.0;
        for i in 1..rhythm.len() {
    
            if current_motion_count > motion_limit {
                expected_motion = if expected_motion == "disjunct" {
                    "conjunct"
                } else {
                    "disjunct"
                };
                motion_limit = if expected_motion == "disjunct" {
                    rand::thread_rng().gen_range(2..8)
                } else {
                    rand::thread_rng().gen_range(2..6)
                };
                current_motion_count = 0;
            }
    
            let current_chord: &Chord
                 = Self::get_chord_at_time(&chords, current_time);
            let mut option = *current_chord.pitches.as_slice()
                .choose(&mut rng).unwrap();
            let last_pitch = *result.last().unwrap_or(&option);
    
            previous_x_notes = result
                .iter()
                .rev()
                .take(std::cmp::min(4, result.len() - 1))
                .map(|p| *p)
                .collect::<Vec<i32>>();
            previous_x_notes.reverse();
    
            let mut tries = 0;
            while previous_x_notes.iter().all(|&pitch| pitch == option)
            || (option - last_pitch).abs() > max_step_size
            || Self::determine_motion(option, last_pitch) 
                != expected_motion
            {
                tries += 1;
                let mut options: Vec<i32> = Vec::new();
                for ap in &current_chord.aps {
                    options.extend(ap);
                }
                let filtered_options: Vec<i32> = options.into_iter()
                    .filter(|p| *p >= span.0 && *p <= span.1)
                    .collect();

                let repeated_options: Vec<i32> = filtered_options.iter()
                    .cloned()
                    .cycle()
                    .take(3 * filtered_options.len())
                    .collect();

                let mut applied_key_options: Vec<i32> = applied_key.aps.iter()
                    .filter(|p| **p >= span.0 && **p <= span.1)
                    .cloned()
                    .collect();

                let mut choices = repeated_options;
                choices.append(&mut applied_key_options);

                option = *choices.choose(&mut rand::thread_rng()).unwrap();
                previous_x_notes = result.iter().rev().take(4)
                    .copied().chain(std::iter::once(option)).collect();
                previous_x_notes.reverse();
                if tries > 80 {
                    break;
                }
            }
            current_motion_count += 1;
            result.push(option);
            current_time += SPACE_VALUES[&rhythm[i]];
        }
        result
    }
    
    
    fn get_chord_at_time(chords: &[Chord], time: f32) -> &Chord {
        chords
            .iter()
            .rev()
            .find(|chord| chord.time.unwrap_or_default().0 <= time)
            .unwrap_or(&chords[0])
    }

    pub fn merge_pitches_with_rhythm(
        pitches: &[i32], rhythm: &[String])
         -> Vec<(i32, f32, f32)> {
        let mut result: Vec<(i32, f32, f32)> = Vec::new();
        let mut time_length: f32 = 0.0;
        
        for i in 0..pitches.len() {
            time_length += SPACE_VALUES[&rhythm[i]];
            result.push((pitches[i], SPACE_VALUES[&rhythm[i]], time_length));
        }
        
        result
    }
}

pub fn clamp_to_range(mut value: i32, range: (i32, i32)) -> i32 {
    let (min, max) = range;
    while value < min || value > max {
        if value < min {
            value += 12
        } else {
            value -= 12
        }
    }
    value
}
