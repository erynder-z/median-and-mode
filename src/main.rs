mod statistics;

fn main() {
    let mut numbers = vec![1, 6, 9, 2, 3, 5, 7, 8, 4, 10, 69, 69, 69, -69];

    // calculate and print the median
    let median = statistics::calculate_median(&mut numbers);
    println!("Median: {}", median);

    // calculate and print the mode
    let mode = statistics::calculate_mode(&numbers);
    println!("Mode: {}", mode);
}
