use crate::day1::solution::calculate_calibration_value_of_file;

mod day1;

fn main() {
    let day1_result = calculate_calibration_value_of_file("./src/day1/puzzle_input.txt");
    if let Ok(day1_solution) = day1_result {
        println!("DAY1 | solution: {}", day1_solution);
    } else {
        println!("DAY1 | error: {:?}", day1_result);
    }
}
