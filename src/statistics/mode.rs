use std::collections::HashMap;

pub fn calculate_mode(numbers: &Vec<i32>) -> i32 {
    let mut frequency_map = HashMap::new();

    // Count the frequency of each number
    for &number in numbers {
        let count = frequency_map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut max_freq = 0;
    let mut mode = 0;

    for (&num, &freq) in &frequency_map {
        if freq > max_freq {
            max_freq = freq;
            mode = num;
        }
    }
    mode
}
