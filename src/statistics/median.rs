

pub fn calculate_median(numbers: &mut Vec<i32>) -> i32 {
    // Sort the vector
    numbers.sort();

    let len = numbers.len();

    if len % 2 == 0 {
        // If the length is even, return the average of the two middle numbers
        let middle_left = numbers[(len / 2) - 1];
        let middle_right = numbers[len / 2];
        i32::from(middle_left + middle_right) / 2
    } else {
        // If the length is odd, return the middle number
        i32::from(numbers[len / 2])
    }
}
