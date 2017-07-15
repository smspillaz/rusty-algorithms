#[derive(Debug)]
struct Result {
    hits: usize,
    pseudo: usize
}

fn mastermind(answer: Vec<char>, guess: Vec<char>) -> Result {
    let mut result = Result {
        hits: 0,
        pseudo: 0
    };

    let mut checked = vec![0, 0, 0, 0];

    for i in 0..4 {
        if answer[i] == guess[i] {
            checked[i] = 1;
            result.hits += 1;
        }
    }

    for i in 0..4 {
        for j in 0..4 {
            if i != j && answer[i] == guess[j] && checked[j] == 0 {
                checked[j] = 1;
                result.pseudo += 1;
            }
        }
    }

    return result;
}

fn main() {
    println!("{:?}", mastermind(vec!['r', 'r', 'r', 'g'],
                                vec!['r', 'g', 'b', 'y']));
}