fn largest_subsequence_sum(input: Vec<i32>, start: usize, end: usize) -> i32 {
    let sum = input[start..end].iter().sum();
    if end == input.len() {
        return sum;
    }

    return std::cmp::max(sum, largest_subsequence_sum(input.clone(), start, end + 1));
}

fn of_all_sums(input: Vec<i32>) -> i32 {
    let sums = (0..input.len()).map(|x| {
        return largest_subsequence_sum(input.clone(), x, x + 1);
    });

    let mut highest = i32::min_value();

    for sum in sums {
        if sum > highest {
            highest = sum;
        }
    }

    return highest;
}

fn main() {
    println!("{:?}", of_all_sums(vec![2, -8, 3, -2, 4, -10]));
}