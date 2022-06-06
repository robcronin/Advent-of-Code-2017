use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input1 = convert_input_to_2d_array("input.txt");
    println!("Day 2, Part 1 is: {}", calculate_check_sum(input1));

    let input2 = convert_input_to_2d_array("input.txt");
    println!(
        "Day 2, Part 2 is: {}",
        calculate_the_evenly_divisible_checksum(input2)
    );
}

fn calculate_check_sum(input: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for row in input {
        let largest = row.iter().max().unwrap();
        let smallest = row.iter().min().unwrap();
        sum += largest - smallest;
    }
    sum
}

fn calculate_the_evenly_divisible_checksum(input: Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for row in input {
        'row_loop: for (index, i) in row.iter().enumerate() {
            for j in row.iter().skip(index + 1) {
                if j % i == 0 {
                    sum += j / i;
                    break 'row_loop;
                } else if i % j == 0 {
                    sum += i / j;
                    break 'row_loop;
                }
            }
        }
    }
    sum
}

// Adapted from: https://github.com/spencermehta/adventofcode2017/blob/main/day02/src/main.rs
fn convert_input_to_2d_array(filename: &str) -> Vec<Vec<u32>> {
    let file = match File::open(filename) {
        Ok(input) => input,
        Err(_) => panic!("File {} doesn't exist", filename),
    };
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let nums: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.split('\t')
                .map(|x| x.parse())
                .map(|x| x.unwrap())
                .collect()
        })
        .collect();

    nums
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_calculates_the_checksum() {
        let result = calculate_check_sum(vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]]);
        assert_eq!(result, 18);
    }
    #[test]
    fn it_calculates_the_checksum2() {
        let result = calculate_check_sum(vec![vec![5, 2, 9, 5], vec![7, 5, 9], vec![1, 4, 6, 5]]);
        assert_eq!(result, 16);
    }

    #[test]
    fn it_calculates_the_evenly_divisible_checksum() {
        let result = calculate_the_evenly_divisible_checksum(vec![
            vec![5, 9, 2, 8],
            vec![9, 4, 7, 3],
            vec![3, 8, 6, 5],
        ]);
        assert_eq!(result, 9);
    }
    #[test]
    fn it_calculates_the_evenly_divisible_checksum2() {
        let result = calculate_the_evenly_divisible_checksum(vec![
            vec![5, 9, 4, 10],
            vec![9, 4, 7, 14],
            vec![31, 8, 60, 5],
        ]);
        assert_eq!(result, 16);
    }
}
