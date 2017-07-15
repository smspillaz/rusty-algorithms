extern crate rand;
use rand::{thread_rng, Rng};

/* Pick one, swap into place */
fn main() {
    let mut array: Vec<i32> = (0..52).collect();
    let mut rng = &mut thread_rng();

    for i in 0..52 {
        /* Pick anywhere from the remaining elements and then swap */
        let swap = rng.gen_range(i, 52);
        let tmp = array[swap];
        array[swap] = array[i];
        array[i] = tmp;
    }

    println!("{:?}", array);
}