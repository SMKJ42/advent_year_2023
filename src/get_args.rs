use std::io::{stdin, stdout, Write};

pub fn get_args() -> Option<(i8, i8)> {
    // take input from user
    println!("enter a day from 1 to 24, followed by a 1 or 2 for the problem:");
    let mut args = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut args)
        .expect("Did not enter a correct string");

    // handle input from user
    args = args.trim().to_string();
    let idx = args.find(' ');
    let args: Vec<&str> = match idx {
        Some(idx) => {
            let arg1 = &args[..idx];
            let arg2 = &args[idx + 1..];
            if arg2.find(' ') != None {
                eprintln!("Too many arguments provided");
                return None;
            }
            vec![arg1, arg2]
        }
        None => {
            eprintln!("Not enough arguments provided");
            return None;
        }
    };

    // parse input from user into i8
    return match (args[0].parse::<i8>(), args[1].parse::<i8>()) {
        (Ok(arg_one), Ok(arg_two)) => Some((arg_one, arg_two)),
        _ => {
            eprintln!("Usage: <number> <1 or 2>");
            None
        }
    };
}
