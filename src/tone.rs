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
        let value = self.value + i;
        let value = if value >= 12 { value - 12 } else { value };
        Tone { value }
    }
}

pub enum Key {
    Min(Tone),
    Maj(Tone),
    // TODO: Add Maj7, Min7, Dim, etc
}

impl Key {
    // Return Some(1), Some(3), Some(5) or None
    // Note: The returned numbers are traditional, 1 actually means 0,
    // 3 is 4 for major or 3 for minor, and 5 is 7.
    pub fn harmonic(&self, t: Tone) -> Option<u8> {
        let (root, third) = match self {
            Key::Min(root) => (root.value, 3),
            Key::Maj(root) => (root.value, 4),
        };
        let interval = if t.value >= root {
            t.value - root
        } else {
            12 + t.value - root
        };
        match interval {
            0 => Some(1),
            n if n == third => Some(3),
            7 => Some(5),
            _ => None,
        }
    }
}

impl FromStr for Key {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Key, Self::Err> {
        if s.ends_with("m") {
            Ok(Key::Min(s[0..s.len() - 1].parse()?))
        } else {
            Ok(Key::Maj(s.parse()?))
        }
    }
}
