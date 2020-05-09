pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::<u32>::new();
    let mut num_buffer = num;

    while num_buffer > 0 {
        digits.push(num_buffer % 10);
        num_buffer = (num_buffer - digits[digits.len() - 1]) / 10;
    }
    
    num == digits.iter()
        .map(|x| x.pow(digits.len() as u32))
        .sum()
}
