fn merge(start: usize, end: usize, mut input: &mut Vec<i32>, mut out: &mut Vec<i32>) {
    if (end - start) == 1 {
        out[start] = input[start];
    } else {
        /* Merge recursively from output into input. Effectively swaps arrays */
        let mid = (end + start) / 2;
        merge(start, mid, &mut out, &mut input);
        merge(mid, end, &mut out, &mut input);

        let mut i = start;
        let mut j = mid;

        /* Go from start to end, exclusive */
        for k in start..end {
            /* Check if the left run head still has elements to go and pick
             * from the left run head if the right run head is exhausted or
             * if we've got a smaller element */
            if i < mid && (j >= end || input[i] < input[j]) {
                out[k] = input[i];
                i += 1;
            } else {
                /* Otherwise pick from the right run head */
                out[k] = input[j];
                j += 1;
            }
        }
    }
}

fn main() {
    let mut collected_iterator: Vec<i32> = (0..10).rev().collect();
    let mut output_iterator: Vec<i32> = collected_iterator.clone();
    merge(0, collected_iterator.len(), &mut collected_iterator, &mut output_iterator);
    println!("Collected (0..1) into: {:?}", collected_iterator);
    println!("Merged (0..1) into: {:?}", output_iterator);
}