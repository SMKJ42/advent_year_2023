use crate::input::read_data;

pub fn problem_1() -> String {
    let data = read_data(1);
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

pub fn problem_2() -> String {
    let data = read_data(1);
    let mut total = 0;
    let data = data.unwrap();

    for line in data.lines() {
        let mut num_s = String::new();
        let chars = line.chars();

        for (i, char) in chars.clone().enumerate() {
            if char.is_digit(10) {
                num_s.push(char);
            } else {
                let substr = chars.clone().into_iter().skip(i).collect::<String>();
                let val = starts_with_num_word(&substr);
                if val != 0 {
                    println!("val: {}", val);
                    num_s.push_str(val.to_string().as_str());
                }
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

fn starts_with_num_word(string: &str) -> i32 {
    let opts = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for opt in opts.iter() {
        if string.starts_with(opt) {
            match *opt {
                "one" => {
                    return 1;
                }
                "two" => {
                    return 2;
                }
                "three" => {
                    return 3;
                }
                "four" => {
                    return 4;
                }
                "five" => {
                    return 5;
                }
                "six" => {
                    return 6;
                }
                "seven" => {
                    return 7;
                }
                "eight" => {
                    return 8;
                }
                "nine" => {
                    return 9;
                }
                _ => {}
            }
        }
    }
    return 0;
}
