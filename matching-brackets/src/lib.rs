pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::<char>::new();
    for c in string.chars() {
        match c {
            '{' | '(' | '[' => stack.push(c),
            '}' => match stack.pop() {
                Some('{') => (),
                _ => {
                    return false;
                }
            },
            ')' => match stack.pop() {
                Some('(') => (),
                _ => {
                    return false;
                }
            },
            ']' => match stack.pop() {
                Some('[') => (),
                _ => {
                    return false;
                }
            },
            _ => (),
        }
    }
    match stack.len() {
        0 => true,
        _ => false,
    }
}
