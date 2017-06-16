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

fn dijkstra(matrix: Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut queue = BinaryHeap::new();
    let mut key: Vec<usize> = matrix.iter().map(|_| usize::MAX).collect();
    let mut parents: Vec<Option<usize>> = matrix.iter().map(|_| None).collect();

    queue.push(State { position: start, weight: 0 });
    while let Some(State { position, weight }) = queue.pop() {
        /* Ignore if we've already done better */
        if weight > key[position] {
            continue;
        }

        for (child_index, child_weight) in matrix[position].iter().enumerate() {
            if *child_weight > 0 && key[child_index] > (*child_weight + weight) {
                key[child_index] = *child_weight + weight;
                parents[child_index] = Some(position);
                queue.push(State {
                    position: child_index,
                    weight: *child_weight + weight
                });
            }
        }
    }

    return key;
}

fn main() {
    let matrix = vec![
        vec![0, 1, 5],
        vec![0, 0, 1],
        vec![0, 0, 1]
    ];
    println!("{:?}", dijkstra(matrix, 0).iter().map(|distance| {
        if *distance == usize::MAX {
            return None
        } else {
            return Some(*distance);
        }
    }).collect::<Vec<Option<usize>>>());
}