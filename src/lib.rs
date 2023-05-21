mod applied_key;
mod chord;
mod chord_progression;
mod forms;
mod melody;
mod motif;
mod presets;
mod rhythm;
mod song;

use wasm_bindgen::prelude::*;

use song::generate_song;


pub fn sanitize_for_javascript(notes: Vec<(i32, f32, f32)>) -> String {
    let converted: Vec<String> = notes
        .iter()
        .map(|(num, float1, float2)| format!("({}, {}, {})", num, float1, float2))
        .collect();
    format!("[{}]", converted.join(", "))
}

#[wasm_bindgen]
pub fn wasm_generate_song() -> String {
    sanitize_for_javascript(generate_song())
}