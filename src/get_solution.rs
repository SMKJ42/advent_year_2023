use crate::{day_1, day_2, day_3, day_4};

pub fn get_solution(day: i8, problem: i8) -> String {
    return match (day, problem) {
        (1, 1) => day_1::problem_1(),
        (1, 2) => day_1::problem_2(),
        (2, 1) => day_2::problem_1(),
        (2, 2) => day_2::problem_2(),
        (3, 1) => day_3::problem_1(),
        (3, 2) => day_3::problem_2(),
        (4, 1) => day_4::problem_1(),
        (4, 2) => day_4::problem_2(),
        _ => {
            return String::from("No solution found for day: ")
                + &day.to_string()
                + "problem: "
                + &problem.to_string()
        }
    };
}
