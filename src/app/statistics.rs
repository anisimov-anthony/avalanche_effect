use crate::app::*;

pub fn percent_difference(s1: &str, s2: &str) -> (f64, Vec<ColoredText>, Vec<ColoredText>) {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    if chars1.len() != chars2.len() {
        return (0.0, Vec::new(), Vec::new());
    }

    let mut older = Vec::new();
    let mut newer = Vec::new();
    let mut diff_count = 0;

    for i in 0..chars1.len() {
        if chars1[i] != chars2[i] {
            diff_count += 1;
            older.push(ColoredText {
                text: chars1[i].to_string(),
                color: Color::Red,
            });
            newer.push(ColoredText {
                text: chars2[i].to_string(),
                color: Color::Yellow,
            });
        } else {
            older.push(ColoredText {
                text: chars1[i].to_string(),
                color: Color::Green,
            });
            newer.push(ColoredText {
                text: chars2[i].to_string(),
                color: Color::Green,
            });
        }
    }

    let percent = if chars1.is_empty() {
        0.0
    } else {
        (diff_count as f64 / chars1.len() as f64) * 100.0
    };

    (percent, older, newer)
}
