trait MinStack {
    fn new() -> Self;
    fn push(&mut self, elem: i32);
    fn pop(&mut self) -> Option<i32>;
    fn min(&self) -> Option<i32>;
}

struct Stack {
    elements: Vec<i32>,
    min_elements: Vec<i32>
}

impl MinStack for Stack {
    fn new() -> Stack {
        return Stack {
            elements: Vec::new(),
            min_elements: Vec::new()
        }
    }

    fn push(&mut self, elem: i32) {
        self.elements.push(elem);

        /* If this element is smaller or
         * equal to than the top
         * element on min_stack, then it is the smallest
         * element seen so far, so add it to min_stack */
        if self.min_elements.len() == 0 ||
           self.min_elements[self.min_elements.len() - 1] >= elem {
            self.min_elements.push(elem);
        }
    }

    fn pop(&mut self) -> Option<i32> {
        let ret = self.elements.pop();
        if let Some(value) = ret {
            if self.min_elements.len() > 0 && self.min_elements[self.min_elements.len() - 1] == value {
                self.min_elements.pop();
            }

            return ret;
        }

        return None;
    }

    fn min(&self) -> Option<i32> {
        if self.min_elements.len() > 0 {
            return Some(self.min_elements[self.min_elements.len() - 1]);
        }

        return None;
    }
}

fn main() {
    let mut stack: Stack = MinStack::new();
    stack.push(3);
    stack.push(2);
    stack.push(1);
    stack.push(0);
    stack.push(0);
    stack.pop();
    stack.pop();
    stack.push(5);
    stack.pop();
    println!("Min element is {:?}", stack.min());
}