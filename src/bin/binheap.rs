fn heapify_down(mut heap: &mut Vec<i32>, n: usize) {
    let mut idx = n;

    loop {
        let left = idx * 2;
        let right = left + 1;

        if heap.len() > right && heap[idx] < heap[right] {
            let tmp = heap[right];
            heap[right] = heap[idx];
            heap[idx] = tmp;
            idx = right;
        } else if heap.len() > left && heap[idx] < heap[left] {
            let tmp = heap[left];
            heap[left] = heap[idx];
            heap[idx] = tmp;
            idx = left;
        } else {
            break;
        }
    }
}

fn heapify_up(mut heap: &mut Vec<i32>, n: usize) {
    let mut idx = n;

    if idx == 0 {
        return;
    }

    loop {
        let parent = idx / 2;

        if parent != idx && heap[idx] > heap[parent] {
            let tmp = heap[parent];
            heap[parent] = heap[idx];
            heap[idx] = tmp;
            idx = parent;
        } else {
            break;
        }
    }
}

struct MaxPrioQueue {
    heap: Vec<i32>
}

impl MaxPrioQueue {
    pub fn new() -> MaxPrioQueue {
        return MaxPrioQueue {
            heap: vec![]
        }
    }

    pub fn push(&mut self, value: i32) {
        self.heap.push(value);
        let last = self.heap.len() - 1;
        heapify_up(&mut self.heap, last);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.heap.len() > 0 {
            let top = self.heap[0];
            if let Some(back) = self.heap.pop() {
                if self.heap.len() > 0 {
                    self.heap[0] = back;
                    heapify_down(&mut self.heap, 0);
                }
            }

            return Some(top);
        }

        return None
    }
}

fn main() {
    let mut queue = MaxPrioQueue::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    println!("{:?}", queue.heap);
    queue.pop();
    println!("{:?}", queue.heap);
}