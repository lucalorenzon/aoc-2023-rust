use std::collections::HashMap;
use regex::{Regex};

pub fn calculate_correct_calibration_line_value(line: &str) -> u32 {
    let first_num_re: Regex = Regex::new(
        r"\d|(zero)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)")
        .expect("valid regex for first_num_re");
    let last_num_re: Regex = Regex::new(
        r".*(\d|(zero)|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine))")
        .expect("valid regex for last_num_re");

    let num: HashMap<&str, u32> = HashMap::from([
        ("0", 0), ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
        ("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
        ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
    ]);

    let first = first_num_re.find(line)
        .map_or(0, |value| num[value.as_str()]);
    let last  = last_num_re.captures(line)
        .map_or(first, |value| num[value.get(1)
            .map_or("0", |value|value.as_str())]);
    first * 10 + last
}

#[cfg(test)]
mod tests {
    use crate::solution_first::{calculate_calibration};
    use crate::solution_second::calculate_correct_calibration_line_value;

    #[test]
    fn calibration_value_of_a_line_with_letter_number() {
        let sample_line = "fiveadthreed1evfivedc4svs";
        let expected_calibration_value = 54u32;

        let actual_calibration_value = calculate_calibration(
            sample_line, calculate_correct_calibration_line_value);

        assert_eq!(expected_calibration_value, actual_calibration_value);
    }

    #[test]
    fn calibration_value_of_a_line_without_any_number_is_zero() {
        let sample_line = "asdevedcsvs";
        let expected_calibration_value = 0u32;

        let actual_calibration_value = calculate_calibration(
            sample_line, calculate_correct_calibration_line_value);

        assert_eq!(expected_calibration_value, actual_calibration_value);
    }
    #[test]
    fn calibration_value_of_a_line_with_one_number_is_that_number_repeated_two_times() {
        let sample_line = "asdevedc4svs";
        let expected_calibration_value = 44u32;

        let actual_calibration_value = calculate_calibration(
            sample_line, calculate_correct_calibration_line_value);

        assert_eq!(expected_calibration_value, actual_calibration_value);
    }

    #[test]
    fn calibration_value_of_a_line_is_the_number_formed_by_first_and_last_digit_in_a_line() {
        let sample_line = "asd1evedc4svs";
        let expected_calibration_value = 14u32;

        let actual_calibration_value = calculate_calibration(
            sample_line, calculate_correct_calibration_line_value);

        assert_eq!(expected_calibration_value, actual_calibration_value);
    }

    #[test]
    fn calibration_of_multiple_line_is_the_sum_of_calibration_of_each_line() {
        let sample_text = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let expected_calibration_value = 142;

        let actual_calibration_value = calculate_calibration(
            sample_text, calculate_correct_calibration_line_value);

        assert_eq!(expected_calibration_value, actual_calibration_value);

    }
}