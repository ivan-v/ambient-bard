mod chord;


fn main() {
    println!("Hello, world!");
    let chord = chord::Chord::new("C".to_owned(), Some("M".to_owned()), None, None);
    // println!("{} {}", chord.root, chord.pitches);
    println!("Pitches: {:?}", chord.pitches);
}

