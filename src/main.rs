mod tone;

use tone::Tone;

fn main() {
    let strings: Vec<Tone> = vec![
        "G".parse().unwrap(),
        "D".parse().unwrap(),
        "A".parse().unwrap(),
        "E".parse().unwrap(),
    ];
    let key: Tone = "F#".parse().unwrap();

    for fret in 0..=12 {
        print!("{:2}:  ", fret);
        for string in &strings {
            print!(
                " {} ",
                key.harmonic(string + fret)
                    .map(|i| format!("{}", i))
                    .unwrap_or(" ".to_string())
            )
        }
        println!();
    }
}
