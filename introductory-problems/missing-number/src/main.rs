use std::io;

fn main() {
    // Read first line of input.
    let mut n: String = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read from stdin.");

    let n: i64 = n.trim().parse().expect("Failed to parse input.");

    // Read second line of input.
    let mut nums: String = String::new();
    io::stdin()
        .read_line(&mut nums)
        .expect("Failed to read from stdin.");

    let vals: Vec<i64> = nums
        .split_whitespace()
        .map(|x| x.parse().expect("Parse error!"))
        .collect();

    let max_sum: i64 = n * (n + 1) / 2;

    let arr_sum: i64 = vals.iter().sum();

    println!("{}", max_sum - arr_sum);
}
