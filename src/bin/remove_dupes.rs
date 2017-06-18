fn remove_dupes(mut input: Vec<char>) -> Vec<char> {
    /* We basically have to O(N^2) here. For every
     * character in the string (using a while loop
     * here since we need to handle mutation of
     * the array as it is iterated) */
    let mut len = input.len();
    let mut i = 0;

    if input.len() == 0 {
        return input;
    }

    while i < len {
        let mut j = i + 1;
        while j < len {
            if input[i] == input[j] {
                /* We found a duplicate, move everything backwards
                 * by one and decrement len */
                for k in (j + 1)..len {
                    input[k - 1] = input[k];
                }

                len -= 1;
            }

            j += 1;
        }

        i += 1;
    }

    return (0..len).map(|i| { input[i] }).collect();
}

fn test(input: &str) -> (&str, Vec<char>) {
    return (input, remove_dupes(input.chars().collect()));
}

fn main() {
    println!("{:?}", test("hello"));
    println!("{:?}", test("abcd"));
    println!("{:?}", test(""));
}