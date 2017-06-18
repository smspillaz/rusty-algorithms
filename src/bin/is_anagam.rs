fn is_anagram(left: &str, right: &str) -> bool {
    let mut map = std::collections::HashMap::new();

    /* Compare lengths, if they are not equal then
     * they cannot be anagrams */
    if left.len() != right.len() {
        return false;
    }

    /* Collect characters - This is O(n log n) over all
     * the characters in the string */
    for c in left.chars() {
        let val = match map.get(&c) {
            Some(v) => v + 1,
            _ => 1
        };
        map.insert(c, val);
    }

    /* Now do the same thing in reverse. Again,
     * O(n log n) */
    for c in right.chars() {
        let (ok, value) = (|c| -> (bool, Option<u32>) {
            if let Some(v) = map.get(&c) {
                if v - 1 == 0 {
                    return (true, None);
                }

                return (true, Some(v - 1));
            }

            return (false, None)
        })(c);

        if !ok {
            return false;
        }

        if let Some(value_insert) = value {
            map.insert(c, value_insert);
        } else {
            map.remove(&c);
        }
    }

    return map.values().len() == 0;
}

fn test<'left, 'right>(left: &'left str, right: &'right str) -> (&'left str, &'right str, bool) {
    return (left, right, is_anagram(left, right));
}

fn main() {
    println!("{:?}", test("hello", "olleh"));
    println!("{:?}", test("", "hello"));
    println!("{:?}", test("", ""));
    println!("{:?}", test("hello", "helll"))
}