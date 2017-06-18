fn has_unique_characters(input: &str) -> bool {
    let mut characters = std::collections::HashSet::new();
    for character in input.chars() {
        match characters.get(&character) {
            Some(_) => return false,
            _ => ()
        };
        characters.insert(character.clone());
    }

    return true;
}

fn has_unique_characters_inplace(input: &str) -> bool {
    /* I think the best we can do here is to double-loop
     * the string, skipping the current index, so
     * O(N^2) */
    for (i, character) in input.chars().enumerate() {
        for (j, cmp) in input.chars().enumerate() {
            if i != j && character == cmp {
                return false;
            }
        }
    }

    return true;
}

fn test(input: &str) -> (&str, bool, bool) {
    return (input, has_unique_characters(input), has_unique_characters_inplace(input));
}

fn main() {
    println!("{:?}", test("hello"));
    println!("{:?}", test("abcd"));
    println!("{:?}", test("abca"));
}