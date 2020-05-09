pub fn abbreviate(phrase: &str) -> String {
    // first letter is always in abbreviation
    // if win[0]ious character is a space then add win[1]ent character to abbrev
    // if win[0]ious word is not uppercase and win[1]ent word is upercase then add win[1]ent character to
    // abbrev
    
    phrase.chars().collect::<Vec<char>>()
        .windows(2)
        .enumerate()
        .filter(|(i, win)| {
            if *i == 0_usize {
                true
            } else if (win[0] == ' ' || win[0] == '-') && ((win[1] >= 'a' && win[1] <= 'z') ||  (win[1] >= 'A' && win[1] <= 'Z')) {
                true
            } else if (win[0] > 'Z' || win[0] < 'A' ) && (win[1] >= 'A' && win[1] <='Z') {
                true
            } else {
                false
            }
        })
        .map(|(i, win)| {
            match i {
                0 => win[0].to_ascii_uppercase(),
                _ => win[1].to_ascii_uppercase()
            }
        })
        .collect::<String>()
}
