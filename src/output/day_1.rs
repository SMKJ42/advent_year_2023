use crate::input::read_data;

pub fn problem_1() -> String {
    let data = read_data(1);
    if data.is_some() {
        let mut total = 0;
        let data = data.unwrap();
        for line in data.lines() {
            let mut num_s = String::new();
            // grab out all valid args
            for char in line.chars() {
                if char.is_digit(10) {
                    num_s.push(char);
                }
            }

            // get the first and last char
            let mut chars = num_s.chars();
            let first_char = chars.next();
            let mut last_char = chars.last();
            if first_char.is_none() {
                continue;
            }
            if last_char.is_none() {
                last_char = first_char;
            }

            // combine the first and last char, then add to the total
            let mut string = String::from(first_char.unwrap());
            string.push(last_char.unwrap());
            total += string.parse::<i32>().unwrap();
        }
        return total.to_string();
    }

    return "Day 1, Problem 1 not solved yet".to_string();
}

pub fn problem_2() -> String {
    return "Day 1, Problem 2 not solved yet".to_string();
}
