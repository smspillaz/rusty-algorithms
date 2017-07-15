use std::collections::{VecDeque};

struct Medianheap {
    left_max: VecDeque<i32>,
    right_min: VecDeque<i32>
}

fn heapify_max(mut maxheap: &mut VecDeque<i32>) {
    let mut idx = maxheap.len() - 1;

    loop {
        let parent = idx / 2;

        if parent != idx && maxheap[idx] > maxheap[parent] {
            let tmp = maxheap[parent];
            maxheap[parent] = maxheap[idx];
            maxheap[idx] = tmp;
            idx = parent;
        } else {
            break;
        }
    }
}

fn heapify_min(mut minheap: &mut VecDeque<i32>) {
    let mut idx = minheap.len() - 1;

    loop {
        let parent = idx / 2;

        if parent != idx && minheap[idx] < minheap[parent] {
            let tmp = minheap[parent];
            minheap[parent] = minheap[idx];
            minheap[idx] = tmp;
            idx = parent;
        } else {
            break;
        }
    }
}

impl Medianheap {
    fn new() -> Medianheap {
        return Medianheap {
            left_max: VecDeque::new(),
            right_min: VecDeque::new()
        }
    }

    fn get_median(&self) -> Option<i32> {
        if self.left_max.len() == 0 && self.right_min.len() == 0 {
            return None
        }

        if self.left_max.len() == self.right_min.len() {
            return Some((self.left_max[0] + self.right_min[0]) / 2);
        }

        if self.left_max.len() > self.right_min.len() {
            return Some(self.left_max[0]);
        }

        return Some(self.right_min[0]);
    }

    fn add(&mut self, value: i32) {
        if let Some(median) = self.get_median() {
            if value >= median {
                self.right_min.push_back(value);
                heapify_min(&mut self.right_min);
            } else {
                self.left_max.push_back(value);
                heapify_max(&mut self.left_max);
            }

            /* Right side heap is too big, pop min values
             * and place on to left side heap until balance
             * is 1 */
            while self.right_min.len() as i32 > self.left_max.len() as i32 {
                if let Some(left_value) = self.right_min.pop_front() {
                    self.left_max.push_back(left_value);
                    heapify_min(&mut self.right_min);
                    heapify_max(&mut self.left_max);
                }
            }
            
            while self.left_max.len() as i32 > self.right_min.len() as i32 {
                if let Some(right_value) = self.left_max.pop_front() {
                    self.right_min.push_back(right_value);
                    heapify_min(&mut self.right_min);
                    heapify_max(&mut self.left_max);
                }
            }
        } else {
            self.left_max.push_back(value);
        }
    }
}

fn main() {
    let numbers: VecDeque<i32> = vec![0, 2, 4, 2, 4, 5, 6, 76, 3, 2, 4, 23, 3, 2, 5, 3, 6].into_iter().collect();
    let mut medianheap = Medianheap::new();

    for number in numbers {
        medianheap.add(number);
        println!("{:?} {:?} {:?} {:?}", number, medianheap.get_median());
    }
}