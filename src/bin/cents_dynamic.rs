fn main() {
    let n = 50i32;
    let mut table: Vec<i32> = (0..n).map(|_| {
        return 0;
    }).collect();

    /* Essentially just the stairs problem, check to see if there is
     * a way to add X cents to a previous solution */
    let approaches = vec![1, 5, 10, 25];
    for i in 0..n {
        for x in 0..approaches.len() {
            if i - approaches[x] >= 0 {
                table[i as usize] += table[(i - approaches[x]) as usize] + 1;
            }
        }
    }

    println!("{:?}", table[(n - 1) as usize]);
}