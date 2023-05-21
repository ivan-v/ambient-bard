use phf::phf_map;

pub static FORMS: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "One-part"      => &["A"],
    "Binary"        => &["A", "B"],
    "Ternary"       => &["A", "B", "A"],
    "Arch"          => &["A", "B", "C", "B", "A"],
    "Sonata"        => &["A", "B", "A"],
    "AABB"          => &["A", "A", "B", "B"],
    "Ballad"        => &["A", "A", "B", "A"],
    "Ballade"       => &["A", "A", "B", "A", "B", "A"],
    "Ballade1"      => &["A", "B", "C", "D", "C", "D", "E", "F", "A", "B"],
    "Ballade2"      => &["A", "B", "C", "D", "C", "D", "E", "A", "B", "A", "B"],
    "Ballade3"      => &["A", "B", "A", "B", "C", "D", "E"],
    "Rondo"         => &["A", "B", "A", "C", "A", "D", "A", "E", "A", "F"],
    "Verse-chorus"  => &["Intro", "A", "B", "A", "C", "B", "C", "B"],
};

pub fn sync_note_durations(notes: Vec<(i32, f32, f32)>, starting_time: Option<f32>) -> Vec<(i32, f32, f32)> {
    let mut result = Vec::new();
    let mut time_length = starting_time.unwrap_or(0.0);

    for i in 0..notes.len() {
        result.push((notes[i].0, notes[i].1, time_length));
        time_length += &notes[i].1;
    }

    result
}
