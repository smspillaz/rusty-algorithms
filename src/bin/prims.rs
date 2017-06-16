use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    weight: usize,
    position: usize
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn prim(matrix: Vec<Vec<usize>>) -> Vec<Option<usize>> {
    let mut queue = BinaryHeap::new();
    let mut key: Vec<usize> = (0..matrix.len()).map(|position| {
        let weight = matrix[position][position];
        if weight > 0 {
            return weight;
        }

        return usize::MAX;
    }).collect();
    let mut parents: Vec<Option<usize>> = (0..matrix.len()).map(|_| None).collect();
    let mut seen: Vec<bool> = (0..matrix.len()).map(|_| false).collect();

    for (position, weight) in key.iter().enumerate() {
        queue.push(State { position, weight: *weight });
    }

    while let Some(State { weight: _, position }) = queue.pop() {
        seen[position] = true;

        /* Go through the adjacent nodes to this one and check if we
         * have something better than what's in the key list. If so,
         * update the key list and push this on to the queue (we'll
         * just end up ignoring the subsequent ones) */
        for (child_position, child_weight) in matrix[position].iter().enumerate() {
            if *child_weight > 0 &&
               !seen[child_position] &&
               key[child_position] > *child_weight {
                key[child_position] = *child_weight;
                parents[child_position] = Some(position);
                queue.push(State {
                    position: child_position,
                    weight: *child_weight
                })
            }
        }
    }

    return parents;
}

fn main() {
    let matrix = vec![
        vec![0, 1, 0],
        vec![3, 0, 1],
        vec![1, 0, 0]
    ];
    println!("{:?}", prim(matrix));
}