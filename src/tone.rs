use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Tone {
    // half-tone increments, 0 = a, 1 = a#, 2 = h, 3 = c, ...
    value: u8,
}

impl FromStr for Tone {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Tone, Self::Err> {
        let value = match s.bytes().next() {
            None => return Err("expected tone, got empty string"),
            Some(s) => match s {
                b'a' | b'A' => 0,
                b'b' | b'B' => 2,
                b'c' | b'C' => 3,
                b'd' | b'D' => 5,
                b'e' | b'E' => 7,
                b'f' | b'F' => 8,
                b'g' | b'G' => 10,
                _ => return Err("Got non-tone initial"),
            },
        };
        let value = match &s[1..] {
            "" => value,
            "b" => {
                if value > 0 {
                    value - 1
                } else {
                    value + 11
                }
            }
            "#" => value + 1,
            _ => return Err("Invalid modifier"),
        };
        Ok(Tone { value })
    }
}

impl Add<u8> for &Tone {
    type Output = Tone;
    fn add(self, i: u8) -> Tone {
        Tone {
            value: (self.value + i) % 12,
        }
    }
}

pub struct Key {
    root: Tone,
    form: KeyForm,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum KeyForm {
    Min,
    Min7,
    Maj,
    Maj7,
    // TODO: Dim, Dim7,
}

impl KeyForm {
    fn harmonic(self, interval: u8) -> Option<u8> {
        match interval {
            0 => Some(1),
            3 if self == Self::Min || self == Self::Min7 => Some(3),
            4 if self == Self::Maj || self == Self::Maj7 => Some(3),
            7 => Some(5),
            10 if self == Self::Min7 || self == Self::Maj7 => Some(7),
            _ => None,
        }
    }
}

impl Key {
    // Return Some(1), Some(3), Some(5) or None
    // Note: The returned numbers are traditional, 1 actually means 0,
    // 3 is 4 for major or 3 for minor, and 5 is 7.
    pub fn harmonic(&self, t: Tone) -> Option<u8> {
        let root = self.root.value;
        let interval = if t.value >= root {
            t.value - root
        } else {
            12 + t.value - root
        };
        self.form.harmonic(interval)
    }
}

impl FromStr for Key {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Key, Self::Err> {
        for (end, form) in &[
            ("m", KeyForm::Min),
            ("m7", KeyForm::Min7),
            ("7", KeyForm::Maj7),
            ("", KeyForm::Maj),
        ] {
            if s.ends_with(end) {
                return Ok(Key {
                    root: s[0..s.len() - end.len()].parse()?,
                    form: *form,
                });
            }
        }
        unreachable!("Anything ends with the empty string")
    }
}
