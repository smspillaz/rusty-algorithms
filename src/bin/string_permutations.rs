fn perm(characters: Vec<char>, prefix: Vec<char>) -> Vec<Vec<char>> {
    if characters.len() > 1 {
        return (0..characters.len()).map(|selected| {
            let mut subset = vec![];
            for i in 0..characters.len() {
                if i != selected {
                    subset.push(characters[i]);
                }
            }

            return perm(subset, [&prefix[..], &vec![characters[selected]]].concat());
        }).flat_map(|s| { s }).collect();
    }

    return vec![[&prefix[..], &vec![characters[0]]].concat()];
}

fn main() {
    let collection = vec!['a', 'b', 'c', 'd', 'e'];
    println!("{:?}", perm(collection, vec![]));
}