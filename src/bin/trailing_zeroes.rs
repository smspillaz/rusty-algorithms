fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    return n * factorial(n - 1);
}

fn trailing_zeroes(mut n: u64) -> i32 {
    let mut trailing = 0i32;

    while n % 10 == 0 {
        trailing += 1;
        n /= 10;
    }

    return trailing;
}

fn main() {
    let fact = factorial(19u64);
    println!("{:?}", fact);
    println!("{:?}", trailing_zeroes(fact));
}