pub fn read_data(day: i8) -> Option<String> {
    let mut path = "src/input/data/day_".to_string() + &day.to_string() + ".txt";

    let data = std::fs::read_to_string(path);
    if data.is_err() {
        println!("Error reading file: {}", data.err().unwrap());
        return None;
    } else {
        return Some(data.unwrap());
    }
}
