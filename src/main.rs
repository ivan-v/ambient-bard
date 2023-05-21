use song::generate_song;

mod applied_key;
mod chord;
mod chord_progression;
mod forms;
mod melody;
mod motif;
mod presets;
mod rhythm;
mod song;

pub fn sanitize_for_javascript(notes: Vec<(i32, f32, f32)>) -> String {
    let converted: Vec<String> = notes
        .iter()
        .map(|(num, float1, float2)| 
            format!("({}, {}, {})", num, float1, float2))
        .collect();
    format!("[{}]", converted.join(", "))
}

fn main() {
    let solution: Vec<(i32, f32, f32)> = generate_song();
    println!("{}", sanitize_for_javascript(solution));
}

