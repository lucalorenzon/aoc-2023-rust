use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn calculate_calibration_value_of_file(file_path: &str, calculate_calibration_line_value:fn(&str)->u32) -> Result<u32, Box<dyn Error>> {
    let file_handle = File::open(file_path)?;
    let mut reader = BufReader::new(&file_handle);
    let file_size = File::metadata(&file_handle).unwrap().len();
    match file_size {
        value if value > 65536 => {
            let mut calibration_value = 0u32;
            let mut lines_iter = reader.lines();
            while let Some(Ok(line)) = lines_iter.next() {
                calibration_value += calculate_calibration_line_value(&line);
            }
            Ok(calibration_value)
        },
        _ => {
            let mut text_content = String::new();
            reader.read_to_string(&mut text_content).unwrap_or(0);
            Ok(calculate_calibration(&text_content, calculate_calibration_line_value))
        }
    }
}

pub fn calculate_calibration(text: &str, calculate_calibration_line_value:fn(&str)->u32) -> u32 {
    text.lines().map(|line| calculate_calibration_line_value(line)).sum()
}

pub fn calculate_calibration_line_value(line: &str) -> u32 {
    let mut numbers_in_line = line.chars()
        .filter_map(|character| character.to_digit(10));

    let first = numbers_in_line.next().unwrap_or(0);
    let last  = numbers_in_line.last().unwrap_or(first);

    first * 10 + last
}

#[cfg(test)]
mod tests {
    use crate::solution_first::{calculate_calibration, calculate_calibration_line_value};

    #[test]
    fn calibration_value_of_a_line_without_any_number_is_zero() {
        let sample_line = "asdevedcsvs";
        let expected_calibration_value = 0u32;

        let actual_calibration_value = calculate_calibration(sample_line, calculate_calibration_line_value);

        assert_eq!(expected_calibration_value, actual_calibration_value);
    }
    #[test]
    fn calibration_value_of_a_line_with_one_number_is_that_number_repeated_two_times() {
        let sample_line = "asdevedc4svs";
        let expected_calibration_value = 44u32;

        let actual_calibration_value = calculate_calibration(sample_line, calculate_calibration_line_value);

        assert_eq!(expected_calibration_value, actual_calibration_value);
    }

    #[test]
    fn calibration_value_of_a_line_is_the_number_formed_by_first_and_last_digit_in_a_line() {
        let sample_line = "asd1evedc4svs";
        let expected_calibration_value = 14u32;

        let actual_calibration_value = calculate_calibration(sample_line, calculate_calibration_line_value);

        assert_eq!(expected_calibration_value, actual_calibration_value);
    }

    #[test]
    fn calibration_of_multiple_line_is_the_sum_of_calibration_of_each_line() {
        let sample_text = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let expected_calibration_value = 142;

        let actual_calibration_value = calculate_calibration(sample_text, calculate_calibration_line_value);

        assert_eq!(expected_calibration_value, actual_calibration_value);

    }

}