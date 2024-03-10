
pub mod solution_first;
pub mod solution_second;

use crate::solution_first::{calculate_calibration_line_value, calculate_calibration_value_of_file};
use crate::solution_second::calculate_correct_calibration_line_value;

fn main() {
    let day1_first_result = calculate_calibration_value_of_file(
        "./day1/puzzle_input.txt",
        calculate_calibration_line_value
    );
    let day1_second_result = calculate_calibration_value_of_file(
        "./day1/puzzle_input.txt",
        calculate_correct_calibration_line_value
    );
    if let Ok(day1_first_solution) = day1_first_result {
        println!("DAY1#1 | solution_first: {}", day1_first_solution);
    } else {
        println!("DAY1#1 | error: {:?}", day1_first_result);
    }
    if let Ok(day1_second_solution) = day1_second_result {
        println!("DAY1#2 | solution_first: {}", day1_second_solution);
    } else {
        println!("DAY1#2 | error: {:?}", day1_second_result);
    }

}
