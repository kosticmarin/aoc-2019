use std::fs;

fn int_code(ops: &mut Vec<i32>) {
    for i in (0..ops.len()).step_by(4) {
        let op_code = ops[i];
        if op_code == 99 {
            break;
        }
        let loc1 = ops[i + 1] as usize;
        let loc2 = ops[i + 2] as usize;
        let loc3 = ops[i + 3] as usize;
        match op_code {
            1 => ops[loc3] = ops[loc1] + ops[loc2],
            2 => ops[loc3] = ops[loc1] * ops[loc2],
            _ => panic!("Invalid op code!"),
        }
    }
}

fn main() {
    let input =
        fs::read_to_string("inputs/in_d2.txt").expect("Input file for day 2 task not found!");
    let ops: Vec<i32> = input
        .split(',')
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
    'outer: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut ops_cloned = ops.to_vec();
            ops_cloned[1] = noun;
            ops_cloned[2] = verb;
            int_code(&mut ops_cloned);
            if ops_cloned[0] == 19_690_720 {
                println!("Noun: {}", noun);
                println!("Verb: {}", verb);
                println!("Result {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_code_example1() {
        let mut input = vec![1, 0, 0, 0, 99];
        int_code(&mut input);
        assert_eq!(vec![2, 0, 0, 0, 99], input);
    }

    #[test]
    fn test_int_code_example2() {
        let mut input = vec![2, 3, 0, 3, 99];
        int_code(&mut input);
        assert_eq!(vec![2, 3, 0, 6, 99], input);
    }

    #[test]
    fn test_int_code_example3() {
        let mut input = vec![2, 4, 4, 5, 99, 0];
        int_code(&mut input);
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], input);
    }

    #[test]
    fn test_int_code_example4() {
        let mut input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        int_code(&mut input);
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], input);
    }
}
