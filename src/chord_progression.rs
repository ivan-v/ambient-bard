use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn grow_minor_chord_progression(
    mut progression: Vec<Vec<&str>>)
     -> Vec<Vec<&str>> {
    let mut rng: rand::rngs::ThreadRng = thread_rng();
    let options: Vec<(&str, Vec<&str>)>;
    if progression.is_empty() {
        options = vec![("i", vec!["suss", "m"])];
        let selected: &(&str, Vec<&str>) = options.choose(&mut rng).unwrap();
        let first_chord: Vec<&str> 
        = vec![selected.0, selected.1.choose(&mut rng).unwrap()];
        return vec![first_chord];
    } else {
        let root: &Vec<&str> = &progression[0];
        if root[0] == "i" && (root[1] == "suss" || root[1] == "m") {
            options = vec![
                ("bVII",  vec!["9"]),
                ("V", vec!["7", "b9", "suss"]),
                ("iv", vec!["m6", "m7", "m9"]),
                ("bIII", vec!["aug"]),
                ("ii", vec!["dim"]),
                ("vii", vec!["dim/2"]),
                ("i", vec!["m/b3"]),
                ("bII", vec!["M"]),
            ];
        } else if root[0] == "ii" && root[1] == "dim" {
            options = vec![
                ("bVI", vec!["6", "M7"]),
                ("iv", vec!["m/b6", "m6/b6", "m7/b6", "m6", "m7", "m9"]),
                ("i", vec!["m/b3"]),
                ("bII", vec!["M"]),
            ];
        } else if root[0] == "vii" && root[1] == "dim/2" {
            options = vec![
                ("bVI", vec!["6", "M7"]),
                ("iv", vec!["m/b6", "m6/b6", "m7/b6", "m6", "m7", "m9"]),
                ("i", vec!["m/b3"]),
                ("bII", vec!["M/4"]),
            ];
        } else if root[0] == "i" && root[1] == "m/b3" {
            options = vec![
                ("bII", vec!["M"]),
                ("iv", vec!["m", "m6", "m7", "m9"]),
            ];
        } else if (root[0] == "iv" && root[1] == "m7")
            || (root[0] == "bII" && root[1] == "M")
        { 
            options = vec![
                ("ii",   vec!["dim"]),
                ("vii",  vec!["dim/2"]),
                ("I",    vec!["7", "b9"]),
                ("iii",  vec!["dim7"]),
                ("bIII", vec!["aug"]),
                ("i",    vec!["m/b3"]),
                ("bVI",  vec!["6", "M7"]),
                ("iv",   vec!["m/b6", "m6/b6", "m7/b6"]),
            ];
        } else if root[0] == "V" && (root[1] == "7" || root[1] == "b9" 
            || root[1] == "suss") {
            options = vec![
                ("iv",   vec!["m", "m6", "m7", "m9"]),
                ("bII",  vec!["M"]),
                ("ii",   vec!["dim"]),
                ("vii",  vec!["dim/2"]),
                ("II",   vec!["7", "b9"]),
                ("#iv",  vec!["dim7"]),
            ];
        } else if root[0] == "II" && (root[1] == "7" || root[1] == "b9") {
            options = vec![
                ("vi",   vec!["7b5"]),
            ];
        } else if root[0] == "bVII" && root[1] == "9" {
            options = vec![
                ("bVI",  vec!["M"]),
            ];
        } else if root[0] == "I" && (root[1] == "7" || root[1] == "b9") {
            options = vec![
                ("V",   vec!["dim7", "m7b5"]),
            ];
        } else if (root[0] == "bIII" && root[1] == "aug") 
            || (root[0] == "i" && root[1] == "m/b3") {
            options = vec![
                ("bii",  vec!["dim"]),
                ("vii",  vec!["dim/2"]),
            ];
        } else if (root[0] == "bVI" && (root[1] == "6" || root[1] == "M7")) 
            || (root[0] == "iv" && (root[1] == "m/b6" || root[1] == "m6/b6" 
            || root[1] == "m7/b6" || root[1] == "m6" || root[1] == "m7" 
            || root[1] == "m9")) {
            options = vec![
                ("bIII", vec!["7", "9", "b9"]),
                ("V",    vec!["m7b5"]),
            ];
        } else if root[0] == "bIII" && 
            (root[1] == "7" || root[1] == "9" || root[1] == "b9") {
            options = vec![(
                ("bvii", vec!["m7b5"])
            )];
        } else {
            return progression;
        }
        let selected: &(&str, Vec<&str>) = options.choose(&mut rng).unwrap();
        let new_chord: Vec<&str> 
            = vec![selected.0, selected.1.choose(&mut rng).unwrap()];
        progression.insert(0, new_chord);
        return progression;
    }
}

pub fn grow_major_chord_progression(
    mut progression: Vec<Vec<&str>>)
     -> Vec<Vec<&str>> {
    let mut rng = thread_rng();
    let options: Vec<(&str, Vec<&str>)>;
    if progression.is_empty() {
        options = vec![("I", vec!["6", "M7", "M9", "suss"])];
        let selected: &(&str, Vec<&str>) = options.choose(&mut rng).unwrap();
        let first_chord: Vec<&str> 
            = vec![selected.0, selected.1.choose(&mut rng).unwrap()];
        return vec![first_chord];
    } else {
        let root = &progression[0];
        if root[0] == "I" && (root[1] == "suss" || root[1] == "M7"
        || root[1] == "M9" || root[1] == "6") {
            options = vec![
                ("V",  vec!["7", "9", "11", "13", "suss"]),
                ("IV", vec!["6", "M7", "m6", "m"]),
                ("iii", vec!["m7"]),
                ("ii", vec!["m7", "m9"]),
                ("bII", vec!["7"]),
                ("IV", vec!["m7"]),
                ("bVII", vec!["9"]),
            ];
        } else if root[0] == "V" && (root[1] == "7" || root[1] == "9"
        || root[1] == "11" || root[1] == "13" || root[1] == "suss") {
            options = vec![
                ("IV", vec!["6", "M7", "m6", "m"]),
                ("ii", vec!["m7", "m9"]),
                ("II", vec!["7", "9", "b9"]),
                ("#IV", vec!["m7b5"]),
                ("I", vec!["M/5"]),
            ];
        } else if root[0] == "ii" {
            options = vec![
                ("IV", vec!["6", "M7", "m6", "m"]),
                ("vi", vec!["m7", "m9"]),
                ("VI", vec!["7", "9", "b9"]),
                ("#I", vec!["dim7"]),
                ("I", vec!["M/4", "dim/b3"]),
            ];
        } else if root[0] == "iii" {
            options = vec![
                ("ii", vec!["m7", "m9"]),
                ("V", vec!["7", "9", "11", "13", "suss"]),
                ("VII", vec!["7", "9", "b9"]),
                ("#II", vec!["dim7"])
            ];
        } else if root[0] == "IV" && (root[1] == "6" || root[1] == "M7" ||
            root[1] == "m6" || root[1] == "m") {
            options = vec![
                ("iii", vec!["m7"]),
                ("vi", vec!["m7", "m9"]),
                ("I", vec!["7", "9", "b9"]),
                ("III", vec!["m7b5"])
            ];
        } else if root[0] == "vi" {
            options = vec![
                ("V", vec!["7", "9", "11", "13", "suss"]),
                ("iii", vec!["m7"]),
                ("III", vec!["7", "9", "b9"]),
                ("#V", vec!["dim7"]),
            ];
        } else if root[0] == "bII" || (root[0] == "IV" && root[1] == "m7")  {
            options = vec![("ii", vec!["m7", "m9"])];
        } else if root[0] == "bVII" { 
            options = vec![("bVI",   vec!["M"])];
        } else if root[0] == "I" && root[1] == "M/3" { 
            options = vec![("IV",   vec!["6", "M7", "m6", "m"])];
        } else if root[0] == "II" {
            options = vec![
                ("I", vec!["m6"]),
                ("V", vec!["M/2"]),
                ("VI", vec!["m7b5/b3"]) // the hardest one to process
            ];
        } else if root[0] == "V" && root[1] == "M/2" {
            options = vec![("I",  vec!["m6"])];
        } else if root[0] == "I" && root[1] == "/5" {
            options = vec![
                ("bVI",   vec!["7"]),
                ("bVII",   vec!["9"]),
                ("#IV",   vec!["m7b5"]),
            ];
        } else if root[0] == "VI" && 
            (root[1] == "7" || root[1] == "9" || root[1] == "b9") {
            options = vec![
                ("III",  vec!["m7b5"]),
            ];
        } else if root[0] == "I" && 
            (root[1] == "7" || root[1] == "9" || root[1] == "b9") {
            options = vec![
                ("V",   vec!["m7"]),
            ];
        } else if root[0] == "VII" {
            options = vec![
                ("#IV",  vec!["m7b5"]),
            ];
        } else if root[0] == "III" && 
            (root[1] == "7" || root[1] == "9" || root[1] == "b9") {
            options = vec![
                ("VII",  vec!["m7b5"]),
            ];
        } else {
            return progression;
        }
        let selected: &(&str, Vec<&str>) = options.choose(&mut rng).unwrap();
        let new_chord: Vec<&str> 
            = vec![selected.0, selected.1.choose(&mut rng).unwrap()];
        progression.insert(0, new_chord);
        return progression;
    }
}
