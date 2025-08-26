use colored::Colorize;

pub fn percent_difference(s1: &str, s2: &str) -> (f64, String, String) {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    if chars1.len() != chars2.len() {
        panic!("len != len");
    }

    let mut older = String::new();
    let mut newer = String::new();

    if chars1.len() == 0 {
        return (0.0, older, newer); // если одна из строк пустая
    }

    let mut diff_count = 0;

    for i in 0..chars1.len() {
        if chars1[i] != chars2[i] {
            diff_count += 1;
            older += &chars1[i].to_string();
            newer += &chars2[i].to_string().yellow().to_string();
        } else {
            older += &chars1[i].to_string().green().to_string();
            newer += &chars2[i].to_string().green().to_string();
        }
    }

    let percent = (diff_count as f64 / chars1.len() as f64) * 100.0;
    (percent, older, newer)
}
