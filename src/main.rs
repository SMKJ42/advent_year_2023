mod get_args;
mod get_solution;
mod input;
mod output;

use output::*;

fn main() {
    let mut args = None;

    while args.is_none() {
        args = get_args::get_args();
    }

    match args {
        Some((day, problem)) => {
            println!("{}", get_solution::get_solution(day, problem));
        }
        None => {
            println!("No solution found");
        }
    }
}
