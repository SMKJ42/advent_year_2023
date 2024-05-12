use crate::input::read_data;

pub fn problem_1() -> String {
    let data = read_data(2);
    let bag = Bag::new(12, 13, 14);
    let mut sum_id = 0;

    for line in data.unwrap().lines() {
        let line = line.split_once(":").unwrap();
        let game_id: i32 = str::parse::<i32>(line.0.split(" ").nth(1).unwrap()).unwrap();
        let round = line.1.split(";");
        let mut valid = true;
        for results in round {
            let results = results.split(",");
            let mut result = Result::new();
            for color_input in results {
                let color_input = color_input.trim().split_once(" ").unwrap();
                let count = str::parse::<i32>(color_input.0).unwrap();
                let color = Color::from_str(color_input.1);
                match color {
                    Color::Red => result.set_red(count),
                    Color::Green => result.set_green(count),
                    Color::Blue => result.set_blue(count),
                }
            }
            if !result.is_valid(&bag) {
                valid = false;
                break;
            }
        }
        if valid {
            println!("Game ID: {}", game_id);
            sum_id += game_id;
        }
    }

    return sum_id.to_string();
}

pub fn problem_2() -> String {
    let data = read_data(2);
    let mut pow_sum = 0;

    for line in data.unwrap().lines() {
        let line = line.split_once(":").unwrap();
        let round = line.1.split(";");
        let mut result = Result::new();
        for results in round {
            let results = results.split(",");
            for color_input in results {
                let color_input = color_input.trim().split_once(" ").unwrap();
                let count = str::parse::<i32>(color_input.0).unwrap();
                let color = Color::from_str(color_input.1);
                match color {
                    // Find way to eliminate the redundancy
                    Color::Red => {
                        if count > result.red {
                            result.set_red(count)
                        }
                    }
                    Color::Green => {
                        if count > result.green {
                            result.set_green(count)
                        }
                    }
                    Color::Blue => {
                        if count > result.blue {
                            result.set_blue(count)
                        }
                    }
                }
            }
        }
        println!("{:?}", result);
        pow_sum += result.red * result.green * result.blue;
    }
    return pow_sum.to_string();
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from_str(s: &str) -> Color {
        match s {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Invalid color"),
        }
    }
}

#[derive(Debug)]
struct Bag {
    red: i32,
    green: i32,
    blue: i32,
}

impl Bag {
    fn new(red: i32, green: i32, blue: i32) -> Bag {
        Bag { red, green, blue }
    }
}

#[derive(Debug)]

struct Result {
    red: i32,
    green: i32,
    blue: i32,
}

impl Result {
    fn new() -> Result {
        Result {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn set_red(&mut self, red: i32) {
        self.red = red;
    }

    fn set_green(&mut self, green: i32) {
        self.green = green;
    }

    fn set_blue(&mut self, blue: i32) {
        self.blue = blue;
    }

    fn is_valid(&self, bag: &Bag) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }
}
