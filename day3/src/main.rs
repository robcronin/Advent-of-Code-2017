fn main() {
    let input = 325489;
    println!("Day 3, Part 1 is: {}", calculate_spiral_distance(input));
}

fn calculate_spiral_distance(start: u32) -> u32 {
    /*
        c = centre
        M = midpoint
        . . M . .
        . . M . .       5 4 3
        M M c M M       6 1 2
        . . M . .       7 8 9
        . . M . .
    */
    if start == 1 {
        return 0;
    }
    let spiral_square_length = get_spiral_square_length(start);
    let spiral_number = spiral_square_length / 2;

    let midpoint_to_centre = spiral_number;

    let distance_between_midpoints = spiral_square_length - 1;
    let entry_offset = spiral_number + 1; // not happy with the justifcation here
    let outer_to_last_midpoint = (start - entry_offset) % distance_between_midpoints;
    let max_outer_midpoint_distance = distance_between_midpoints / 2;

    if outer_to_last_midpoint > max_outer_midpoint_distance {
        let outer_to_closest_midpoint = distance_between_midpoints - outer_to_last_midpoint;
        return midpoint_to_centre + outer_to_closest_midpoint;
    }
    midpoint_to_centre + outer_to_last_midpoint
}

fn get_spiral_square_length(start: u32) -> u32 {
    let floor_sqrt = f32::sqrt((start - 1) as f32) as u32;
    if floor_sqrt % 2 == 0 {
        return floor_sqrt + 1;
    }
    floor_sqrt + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_calculates_spiral_distance_for_1() {
        let result = calculate_spiral_distance(1);
        assert_eq!(result, 0);
    }
    #[test]
    fn it_calculates_spiral_distance_for_12() {
        let result = calculate_spiral_distance(12);
        assert_eq!(result, 3);
    }
    #[test]
    fn it_calculates_spiral_distance_for_23() {
        let result = calculate_spiral_distance(23);
        assert_eq!(result, 2);
    }
    #[test]
    fn it_calculates_spiral_distance_for_a_square() {
        let result = calculate_spiral_distance(9);
        assert_eq!(result, 2);
    }
    #[test]
    fn it_calculates_spiral_distance_for_a_square2() {
        let result = calculate_spiral_distance(25);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_calculates_spiral_distance_for_1024() {
        let result = calculate_spiral_distance(1024);
        assert_eq!(result, 31);
    }
    #[test]
    fn it_calculates_spiral_distance_for_26() {
        let result = calculate_spiral_distance(26);
        assert_eq!(result, 5);
    }
    #[test]
    fn it_calculates_part_1() {
        let result = calculate_spiral_distance(325489);
        assert_eq!(result, 552);
    }
}
