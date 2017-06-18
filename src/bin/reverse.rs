fn reverse_keeplast(mut chars: Vec<char>) -> Vec<char> {
    let len = chars.len();
    for i in 0..((len - 1) / 2) {
        let j = (len - 2) - i;
        let tmp = chars[i];
        chars[i] = chars[j];
        chars[j] = tmp;
    }

    return chars;
}

fn test(input: &str) -> (&str, Vec<char>) {
    return (input, reverse_keeplast(input.chars().collect()));
}

fn main() {
    println!("{:?}", test("hello0"));
}