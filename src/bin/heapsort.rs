fn sift_down(mut heap: &mut Vec<i32>, i: usize, len: usize) {
  /* Compare with left and right child and take the greater of the two */
  let left = i * 2;
  let right = left + 1;

  if left >= len {
    return;
  }

  /* Has a right child, need to check both */
  if right < len {
    if heap[left] > heap[right] && heap[left] > heap[i] {
      let tmp = heap[i];
      heap[i] = heap[left];
      heap[left] = tmp;

      /* Now recurse into the left child */
      sift_down(heap, left, len);
    } else if heap[right] > heap[left] && heap[right] > heap[i] {
      let tmp = heap[i];
      heap[i] = heap[right];
      heap[right] = tmp;

      /* Now recurse into the right child */
      sift_down(heap, right, len);
    }
  } else {
    if heap[left] > heap[i] {
      let tmp = heap[i];
      heap[i] = heap[left];
      heap[left] = tmp;

      /* Now recurse into the left child */
      sift_down(heap, left, len);
    }
  }
}

fn rectify_heap(mut heap: &mut Vec<i32>) {
  let len = heap.len();
  let sz = len / 2;

  for i in (0..sz).rev() {
    sift_down(heap, i, len);
  }
}

// [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// [1, 2, 3, 4, 10, 6, 7, 8, 9, 5]
// [1, 2, 3, 9, 10, 6, 7, 8, 4, 5]
// [1, 2, 7, 9, 10, 6, 3, 8, 4, 5]
// [1, 10, 7, 9, 2, 6, 3, 8, 4, 5]
// [1, 10, 7, 9, 5, 6, 3, 8, 4, 2]
// [10, 1, 7, 9, 5, 6, 3, 8, 4, 2]
// [10, 9, 7, 1, 5, 6, 3, 8, 4, 2]
// [10, 9, 7, 8, 5, 6, 3, 1, 4, 2]

// [2, 9, 7, 8, 5, 6, 3, 1, 4, 10]
// [9, 2, 7, 8, 5, 6, 3, 1, 4, 10]
// [9, 8, 7, 2, 5, 6, 3, 1, 4, 10]
// [9, 8, 7, 4, 5, 6, 3, 1, 2, 10]

// [2, 8, 7, 4, 5, 6, 3, 1, 9, 10]
// [8, 2, 7, 4, 5, 6, 3, 1, 9, 10]
// [8, 4, 7, 2, 5, 6, 3, 1, 9, 10]
// [8, 4, 7, 2, 5, 6, 3, 1, 9, 10]

// [1, 4, 7, 2, 5, 6, 3, 8, 9, 10]
// [7, 4, 1, 2, 5, 6, 3, 8, 9, 10]
// [7, 4, 6, 2, 5, 1, 3, 8, 9, 10]

// [3, 4, 6, 2, 5, 1, 7, 8, 9, 10]
// [6, 4, 3, 2, 5, 1, 7, 8, 9, 10]

// [1, 4, 3, 2, 5, 6, 7, 8, 9, 10]
// [4, 1, 3, 2, 5, 6, 7, 8, 9, 10]
// [4, 3, 1, 2, 5, 6, 7, 8, 9, 10]

// [2, 3, 1, 4, 5, 6, 7, 8, 9, 10]
// [3, 2, 1, 4, 5, 6, 7, 8, 9, 10]

// [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

fn order_from_heap(mut heap: &mut Vec<i32>) {
  for size in (1..heap.len()).rev() {
    // Swap the last element of the heap with
    // the first element of the array, then sift
    // down (treating the heap as though it only
    // extends to size)
    let tmp = heap[size];
    heap[size] = heap[0];
    heap[0] = tmp;

    sift_down(heap, 0, size);
  }
}

fn main() {
  let mut heap = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  rectify_heap (&mut heap);
  order_from_heap (&mut heap);

  println!("{:?}", heap);
}