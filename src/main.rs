mod tone;

use tone::{Key, Tone};

fn main() {
    let strings: Vec<Tone> = ["G", "D", "A", "E"] // Mandolin
        // ["E", "A", "D", "G", "B", "E"] // Guitar
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let key_s = "C#m";
    let key = key_s.parse::<Key>().unwrap();

    println!(
        "<style>\
         \ntd {{\
         \n  border: 1px solid lightgray;\
         \n  width: 2em;\
         \n}}\
         \ntable {{\
         \n  border-collapse: collapse;\
         \n}}\
         \ntr:nth-child(1) td {{\
         \n  border: 0;\
         \n}}\
         \nth {{\
         \n  font-size: 80%;\
         \n  font-weight: normal;\
         \n  font-style: italic;\
         \n}}\
         \ntd {{\
         \n  text-align: center;\
         \n  padding: 0;\
         \n}}"
    );
    for i in 2..=13 {
        println!(
            "tr:nth-child({}) {{ height: {}em }}",
            i,
            1.4 * 1.07_f32.powi(13 - i)
        );
    }
    println!("</style>");
    println!("<table>");
    println!("<caption>{}</caption>", key_s);
    for fret in 0..=12 {
        print!("<tr><th>{:2}:</th>", fret);
        for string in &strings {
            print!(
                "<td>{}</td>",
                key.harmonic(string + fret)
                    .map(|i| i.to_string())
                    .unwrap_or(" ".to_string())
            )
        }
        println!("</tr>");
    }
    println!("</table>");
}
