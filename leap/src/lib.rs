pub fn is_leap_year(year: u64) -> bool {
    // year % 4 == 0
    //      and not year % 100 == 0
    //          or year % 100 == 0 and  year % 400 == 0
    // A & (!B | B & C) 
    year % 4 == 0 && (year % 100 != 0 || year % 100 == 0 && year % 400 == 0)
}
