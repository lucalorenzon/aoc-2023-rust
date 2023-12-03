use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn calculate_calibration_value_of_file(file_path: &str) -> Result<u32, Box<dyn Error>> {
    let file_handle = File::open(file_path)?;
    let mut reader = BufReader::new(&file_handle);
    let file_size = File::metadata(&file_handle).unwrap().len();
    match file_size {
        value if value > 65536 => {
            let mut calibration_value = 0u32;
            let mut lines_iter = reader.lines();
            while let Some(Ok(line)) = lines_iter.next() {
                if let Ok(value) = calculate_calibration_line_value(&line).parse::<u32>() {
                    calibration_value += value;
                }
            }
            Ok(calibration_value)
        },
        _ => {
            let mut text_content = String::new();
            reader.read_to_string(&mut text_content).unwrap_or(0);
            Ok(calculate_calibration(&text_content))
        }
    }
}

pub fn calculate_calibration(text: &str) -> u32 {
    let mut calibration_value = 0u32;
    let mut lines_iter = text.lines();
    while let Some(line) = lines_iter.next() {
        let line_calibration_value = calculate_calibration_line_value(line);
        if let Ok(value) = line_calibration_value.parse::<u32>() {
            calibration_value += value;
        }
    }
    calibration_value
}

fn calculate_calibration_line_value(line: &str) -> String {
    let (first_digit, last_digit) = get_first_and_last_digit(line);
    let line_calibration_value = vec![first_digit, last_digit].into_iter()
        .map(|digit| digit.to_string())
        .fold(String::from(""),
              |mut acc, digit| {
                  acc.push_str(&digit);
                  acc
              });
    line_calibration_value
}

fn get_first_and_last_digit(line: &str) -> (u32, u32) {
    let numbers_in_line: Vec<u32> = line.chars()
        .filter(|character| character.is_numeric())
        .map(|c| c.to_digit(10))
        .filter_map(|digit| digit)
        .collect();

    match numbers_in_line {
        array if(array.len()>=2) => (array[0], array[array.len()-1]),
        array if(array.len()==1) => (array[0], array[0]),
        _ => (0,0),
    }
}

#[cfg(test)]
mod tests {
    use crate::day1::solution::calculate_calibration;

    #[test]
    fn calibration_value_of_a_line_without_any_number_is_zero() {
        let sample_line = "asdevedcsvs";
        let expected_calibration_value = 0u32;

        let actual_calibration_value = calculate_calibration(sample_line);

        assert_eq!(expected_calibration_value, actual_calibration_value);
    }
    #[test]
    fn calibration_value_of_a_line_with_one_number_is_that_number_repeated_two_times() {
        let sample_line = "asdevedc4svs";
        let expected_calibration_value = 44u32;

        let actual_calibration_value = calculate_calibration(sample_line);

        assert_eq!(expected_calibration_value, actual_calibration_value);
    }

    #[test]
    fn calibration_value_of_a_line_is_the_number_formed_by_first_and_last_digit_in_a_line() {
        let sample_line = "asd1evedc4svs";
        let expected_calibration_value = 14u32;

        let actual_calibration_value = calculate_calibration(sample_line);

        assert_eq!(expected_calibration_value, actual_calibration_value);
    }

    #[test]
    fn calibration_of_multiple_line_is_the_sum_of_calibration_of_each_line() {
        let sample_text = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let expected_calibration_value = 142;

        let actual_calibration_value = calculate_calibration(sample_text);

        assert_eq!(expected_calibration_value, actual_calibration_value);

    }

}