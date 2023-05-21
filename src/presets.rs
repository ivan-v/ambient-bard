use crate::applied_key::AppliedKey;

pub struct Presets {
    pub meter: [f32; 2],
    pub rhythm_intensity: String,
    pub max_step_size: i32,
    pub span: (i32, i32),
    pub applied_key: AppliedKey<'static>,
    pub beats: Vec<f32>,
    pub desired_chord_progression_length: i32,
    pub is_major: bool,
    pub measure_count: i32
}

impl Presets {
    pub fn new(
        meter: [f32; 2],
        rhythm_intensity: String,
        max_step_size: i32,
        span: (i32, i32),
        applied_key: AppliedKey<'static>,
        beats: Vec<f32>,
        desired_chord_progression_length: i32,
        is_major: bool,
        measure_count: i32
    ) -> Self {
        Self {
            meter: meter,
            rhythm_intensity: rhythm_intensity,
            max_step_size: max_step_size,
            span: span,
            applied_key: applied_key,
            beats: beats,
            desired_chord_progression_length: desired_chord_progression_length,
            is_major: is_major,
            measure_count: measure_count
        }
    }
}

impl Clone for Presets {
    fn clone(&self) -> Self {
        Presets {
            meter: self.meter,
            rhythm_intensity: self.rhythm_intensity.clone(),
            max_step_size: self.max_step_size,
            span: self.span,
            applied_key: self.applied_key.clone(),
            beats: self.beats.clone(),
            desired_chord_progression_length: self.desired_chord_progression_length,
            is_major: self.is_major,
            measure_count: self.measure_count,
        }
    }
}