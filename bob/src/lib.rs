use regex::Regex;

pub fn reply(message: &str) -> &str {
    let yelled_question = Regex::new(r"^[A-Z \d,\)\(:;'\n\r!\.%\^\*@#\$]*[A-Z]+[A-Z \d,\)\(:;'\n\r%\^\*@#\$]*\?[ ]*$");
    let question = Regex::new(r"^[A-z \d,:;\(\)'\n\r!\.%\^\*@#\$]+\?[ ]*$");
    let all_caps = Regex::new(r"^[A-Z \d,:\(\);'\.!\n\r%\^\*@#\$]*[A-Z]+[A-Z \d,:;'\.!\n\r!\.%\^\*@#\$]*[ ]*$");
    let silence = Regex::new(r"^\s*$");
    // test empty string case;
    
    if yelled_question.unwrap().is_match(message) { return "Calm down, I know what I'm doing!" }
    else if question.unwrap().is_match(message) { return "Sure."}
    else if all_caps.unwrap().is_match(message) { return "Whoa, chill out!"}
    else if silence.unwrap().is_match(message) { return "Fine. Be that way!"}
    else {return "Whatever."} 

}

