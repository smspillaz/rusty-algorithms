fn take_lowest(mut stack: &mut Vec<i32>,
               mut carry: &mut Vec<i32>,
               mut sorted: &mut Vec<i32>) {
    let mut lowest = i32::max_value();
    let mut index = 0;
    let mut current_index = 0i32;

    /* Go down the stack and grab the lowest value */
    while stack.len() > 0 {
        let val = stack.pop();
        if let Some(val_int) = val {
            if val_int <= lowest {
                lowest = val_int;
                index = current_index;
            }

            carry.push(val_int);
            current_index += 1;
        }
    }

    /* Push that value on to the stack */
    sorted.push(lowest);

    /* Now push the other values on to the
     * existing stack, but not the lowest */
    current_index = (carry.len() - 1) as i32;
    while carry.len() > 0 {
        if let Some(val) = carry.pop() {
            if current_index != index {
                stack.push(val);
            }
            current_index -= 1;
        }
    }

    carry.pop();
}


fn main() {
    let mut stack = vec![5, 4, 3, 2, 1];
    let mut sorted = vec![];
    let mut carry = vec![];

    let len = stack.len();

    while sorted.len() != len {
        take_lowest(&mut stack, &mut carry, &mut sorted);
    }

    println!("{:?}", sorted);
}