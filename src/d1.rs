use std::fs;

fn fuel_for_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        0
    } else {
        fuel + fuel_for_fuel(fuel)
    }
}

fn fuel_counter_upper(input: String) -> i32 {
    input
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .map(fuel_for_fuel)
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
        assert_eq!(2, fuel_counter_upper("12".to_owned()));
    }

    #[test]
    fn test_fuel_counter_upper_14() {
        assert_eq!(2, fuel_counter_upper("14".to_owned()));
    }

    #[test]
    fn test_fuel_counter_upper_1969() {
        assert_eq!(966, fuel_counter_upper("1969".to_owned()));
    }

    #[test]
    fn test_fuel_counter_upper_100756() {
        assert_eq!(50346, fuel_counter_upper("100756".to_owned()));
    }

    #[test]
    fn test_fuel_counter_upper_sum_of_examples() {
        let input = "12\n14\n1969\n100756\n".to_owned();
        let expected = 2 + 2 + 966 + 50346;
        assert_eq!(expected, fuel_counter_upper(input));
    }
}
