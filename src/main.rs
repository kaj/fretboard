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
         \n  border: 1px solid lightgray;\
         \n  width: 2em;\
         \n}}\
         \ntable {{\
         \n  border-collapse: collapse;\
         \n}}\
         \ntr:nth-child(1) td {{\
         \n  border: 0;\
         \n  border-bottom: double 4px black;\
         \n}}\
         \nth {{\
         \n  padding-right: .2em;\
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

    let mut strings = &guitar;
    for arg in args().skip(1) {
        match arg.as_str() {
            "mandolin" => {
                strings = &mandolin;
            }
            "guitar" => {
                strings = &guitar;
            }
            key_s => {
                let key = match key_s.parse::<Key>() {
                    Ok(key) => key,
                    Err(e) => {
                        eprintln!("Exepected instrument or chord, got {:?}: {}", key_s, e);
                        exit(1);
                    }
                };
                println!("<div class='chord'><table>");
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
                                .unwrap_or(" ".to_string())
                        )
                    }
                    println!("</tr>");
                }
                println!("</table></div>");
            }
        }
    }
}
