use std::fs;

fn fuel_counter_upper(input: String) -> i32 {
    input
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .map(|n| (n / 3) - 2)
        .sum()
}

fn main() {
    let input = fs::read_to_string("inputs/in_d1.txt").expect("Missing input file for task 1.");
    println!("{}", fuel_counter_upper(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_counter_upper_12() {
        assert_eq!(2i32, fuel_counter_upper("12".to_owned()));
    }

    #[test]
    fn test_fuel_counter_upper_14() {
        assert_eq!(2i32, fuel_counter_upper("14".to_owned()));
    }

    #[test]
    fn test_fuel_counter_upper_1969() {
        assert_eq!(654i32, fuel_counter_upper("1969".to_owned()));
    }

    #[test]
    fn test_fuel_counter_upper_100756() {
        assert_eq!(33583i32, fuel_counter_upper("100756".to_owned()));
    }

    #[test]
    fn test_fuel_counter_upper_sum_of_examples() {
        let input = "12\n14\n1969\n100756\n".to_owned();
        let expected = 2i32 + 2i32 + 654i32 + 33583i32;
        assert_eq!(expected, fuel_counter_upper(input));
    }
}
