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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_difference_identical_strings() {
        let (percent, old_colored, new_colored) = percent_difference("abcdef", "abcdef");
        assert_eq!(percent, 0.0);
        assert_eq!(old_colored.len(), 6);
        assert_eq!(new_colored.len(), 6);

        for colored in &old_colored {
            assert!(matches!(colored.color, Color::Green));
        }
        for colored in &new_colored {
            assert!(matches!(colored.color, Color::Green));
        }
    }

    #[test]
    fn test_percent_difference_completely_different() {
        let (percent, old_colored, new_colored) = percent_difference("aaaaaa", "bbbbbb");
        assert_eq!(percent, 100.0);
        assert_eq!(old_colored.len(), 6);
        assert_eq!(new_colored.len(), 6);

        for colored in &old_colored {
            assert!(matches!(colored.color, Color::Red));
        }
        for colored in &new_colored {
            assert!(matches!(colored.color, Color::Yellow));
        }
    }

    #[test]
    fn test_percent_difference_half_different() {
        let (percent, old_colored, new_colored) = percent_difference("aaabbb", "aaaccc");
        assert_eq!(percent, 50.0);
        assert_eq!(old_colored.len(), 6);
        assert_eq!(new_colored.len(), 6);

        assert!(matches!(old_colored[0].color, Color::Green));
        assert!(matches!(old_colored[1].color, Color::Green));
        assert!(matches!(old_colored[2].color, Color::Green));
        assert!(matches!(old_colored[3].color, Color::Red));
        assert!(matches!(old_colored[4].color, Color::Red));
        assert!(matches!(old_colored[5].color, Color::Red));
    }

    #[test]
    fn test_percent_difference_different_lengths() {
        let (percent, old_colored, new_colored) = percent_difference("abc", "abcdef");
        assert_eq!(percent, 0.0);
        assert_eq!(old_colored.len(), 0);
        assert_eq!(new_colored.len(), 0);
    }

    #[test]
    fn test_percent_difference_empty_strings() {
        let (percent, old_colored, new_colored) = percent_difference("", "");
        assert_eq!(percent, 0.0);
        assert_eq!(old_colored.len(), 0);
        assert_eq!(new_colored.len(), 0);
    }

    #[test]
    fn test_percent_difference_one_char_different() {
        let (percent, old_colored, new_colored) = percent_difference("abcd", "abXd");
        assert!((percent - 25.0).abs() < 0.01);
        assert_eq!(old_colored.len(), 4);
        assert_eq!(new_colored.len(), 4);

        assert!(matches!(old_colored[2].color, Color::Red));
        assert_eq!(old_colored[2].text, "c");
        assert!(matches!(new_colored[2].color, Color::Yellow));
        assert_eq!(new_colored[2].text, "X");
    }

    #[test]
    fn test_percent_difference_unicode() {
        let (percent, old_colored, new_colored) = percent_difference("🔥🔥", "🔥💧");
        assert_eq!(percent, 50.0);
        assert_eq!(old_colored.len(), 2);
        assert_eq!(new_colored.len(), 2);
    }

    #[test]
    fn test_percent_difference_text_content() {
        let (percent, old_colored, new_colored) = percent_difference("hello", "world");
        assert_eq!(percent, 80.0);

        assert_eq!(old_colored[0].text, "h");
        assert_eq!(old_colored[1].text, "e");
        assert_eq!(old_colored[2].text, "l");
        assert_eq!(old_colored[3].text, "l");
        assert_eq!(old_colored[4].text, "o");

        assert_eq!(new_colored[0].text, "w");
        assert_eq!(new_colored[1].text, "o");
        assert_eq!(new_colored[2].text, "r");
        assert_eq!(new_colored[3].text, "l");
        assert_eq!(new_colored[4].text, "d");
    }
}
