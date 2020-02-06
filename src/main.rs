mod tone;

use std::env::args;
use std::process::exit;
use std::ptr;
use tone::{Key, Tone};

fn main() {
    let mandolin: Vec<Tone> = ["G", "D", "A", "E"]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let guitar: Vec<Tone> = ["E", "A", "D", "G", "B", "E"]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();

    println!(
        "<!doctype html>\
         \n<meta charset='utf-8'>\
         \n<style>\
         \ndiv.chord {{ display: inline-block; padding: 1em; }}\
         \ncaption {{ font: italic bold 1.2em serif }}\
         \ntd {{\
         \n  border: 0 solid gray;\
         \n  border-width: 1px 0;
         \n  padding: 0;\
         \n  text-align: center;\
         \n  width: 2em;\
         \n}}\
         \n.guitar td {{\
         \n  background-image: linear-gradient(90deg, transparent 46%, #fd6, #fd6, transparent 54%);\
         \n}}\
         \n.guitar td:nth-child(n + 5) {{\
         \n  background-image: linear-gradient(90deg, transparent 48%, #fd6, transparent 52%);\
         \n}}\
         \n.mandolin td {{\
         \n  background-image: linear-gradient(90deg, transparent 40%, #fd6, transparent, transparent, transparent, #fd6, transparent 60%);\
         \n}}\
         \n.mandolin td:nth-child(n + 4) {{\
         \n  background-image: linear-gradient(90deg, transparent 41%, #fd6, transparent, transparent, transparent, transparent, #fd6, transparent 59%);\
         \n}}\
         \ntd:nth-child(2) {{ border-left: 1px solid gray }}\
         \ntd:last-child {{ border-right: 1px solid gray }}\
         \ntable {{\
         \n  border-collapse: collapse;\
         \n}}\
         \n.chord tr:nth-child(1) td {{\
         \n  background-image: none;\
         \n  border: 0;\
         \n  border-bottom: double 4px black;\
         \n}}\
         \nth {{\
         \n  padding-right: .2em;\
         \n}}\
         "
    );
    for i in 2..=13 {
        println!(
            "tr:nth-child({}) {{ height: {}em }}",
            i,
            1.4 * 1.07_f32.powi(13 - i)
        );
    }
    println!("</style>");

    let mut strings = &guitar;
    let mut instrument = "guitar";
    for arg in args().skip(1) {
        match arg.as_str() {
            "mandolin" => {
                strings = &mandolin;
                instrument = "mandolin";
            }
            "guitar" => {
                strings = &guitar;
                instrument = "guitar";
            }
            key_s => {
                let key = match key_s.parse::<Key>() {
                    Ok(key) => key,
                    Err(e) => {
                        eprintln!("Exepected instrument or chord, got {:?}: {}", key_s, e);
                        exit(1);
                    }
                };
                println!("<div class='chord {}'><table>", instrument);
                println!("<caption>{}</caption>", key_s);
                for fret in 0..=12 {
                    print!(
                        "<tr><th>{}</th>",
                        match fret {
                            3 | 5 | 7 => '·',
                            9 if ptr::eq(strings, &guitar) => '·',
                            10 if ptr::eq(strings, &mandolin) => '·',
                            12 => ':',
                            _ => ' ',
                        }
                    );
                    for string in strings {
                        print!(
                            "<td>{}</td>",
                            key.harmonic(string + fret)
                                .map(|i| i.to_string())
                                .unwrap_or("".to_string())
                        )
                    }
                    println!("</tr>");
                }
                println!("</table></div>");
            }
        }
    }
}
