#[derive(Clone, Debug)]
struct Usage {
    quarters: usize,
    dimes: usize,
    nickels: usize,
    pennies: usize
}

fn cents(money: Usage, n: usize) -> Vec<Usage> {
    let current = money.quarters * 25 +
                  money.dimes * 10 +
                  money.nickels * 5 +
                  money.pennies;

    if current == n {
        return vec![money];
    }

    let mut next = vec![];
    if current + 25 <= n {
        next = [&next[..], &cents(Usage {
            quarters: money.quarters + 1,
            dimes: money.dimes,
            nickels: money.nickels,
            pennies: money.pennies
        }, n)[..]].concat();
    }

    if current + 10 <= n {
        return [&next[..], &cents(Usage {
            quarters: money.quarters,
            dimes: money.dimes + 1,
            nickels: money.nickels,
            pennies: money.pennies
        }, n)[..]].concat();
    }

    if current + 5 <= n {
        return [&next[..], &cents(Usage {
            quarters: money.quarters,
            dimes: money.dimes,
            nickels: money.nickels + 5,
            pennies: money.pennies
        }, n)[..]].concat();
    }

    if current + 1 <= n {
        return [&next[..], &cents(Usage {
            quarters: money.quarters,
            dimes: money.dimes,
            nickels: money.nickels,
            pennies: money.pennies + 1
        }, n)[..]].concat();
    }

    return next;
}

fn main() {
    println!("{:?}", cents(Usage {
        quarters: 0,
        dimes: 0,
        nickels: 0,
        pennies: 0
    }, 100));
}