fn partition(mut array: &mut Vec<i32>, pivot: usize, end: usize) -> usize {
    let pivot_value = array[pivot];
    let mut next_pivot = pivot + 1;

    /* Starting from the pivot index, we keep swapping around the
     * candidate lower bound with the array value. Every time we swap
     * we increment the candidate lower bound. This eventually gets us
     * to our new pivot, where we then swap the old pivot value with
     * the new one and return it */
    for j in pivot..end {
        if array[j] < pivot_value {
            let tmp = array[next_pivot];
            array[next_pivot] = array[j];
            array[j] = tmp;
            next_pivot += 1;
        }
    }

    next_pivot -= 1;
    array[pivot] = array[next_pivot];
    array[next_pivot] = pivot_value;
    return next_pivot;
}

fn quicksort(mut array: &mut Vec<i32>, start: usize, end: usize) {
    if end - start > 1 {
        /* Find a pivot point */
        let pivot = partition(&mut array, start, end);

        /* Recursively sort from the start to pivot - 1
         * and pivot + 1 to end. Note that this does not include
         * the pivot since that doesn't get sorted */
        quicksort(array, start, pivot);
        quicksort(array, pivot + 1, end);
    }
}

fn main() {
    let mut collected_iterator: Vec<i32> = (0..10).rev().collect();
    println!("Collected (0..1) into: {:?}", collected_iterator);
    let end = collected_iterator.len();
    quicksort(&mut collected_iterator, 0, end);
    println!("Merged (0..1) into: {:?}", collected_iterator);
}