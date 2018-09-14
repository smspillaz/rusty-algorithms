/* Strassen's algorithm is another way to multiply two square matrices
 * that is faster than the O(n^3) method in standard matrix multiplication */

/* Matrices are assumed to be in column-major format */
fn naive(a: Vec<Vec<f32>>, b: Vec<Vec<f32>>) -> (u32, Vec<Vec<f32>>) {
  /* Compatibility check */
  if a[0].len() != b.len() {
    panic!("Matrices have to be the same size");
  }

  let n = a[0].len();
  let m = a.len();
  let p = b[0].len();
  let mut ops = 0;
  let mut result = vec![vec![0.0f32; m]; p];

  for i in 0..m {
    for j in 0..p {
      for k in 0..n {
        ops += 1;
        result[j][i] += a[k][j] * b[i][k];
      }
    }
  }

  (ops, result)
}

fn main() {
  println!("naive: {:?}", naive(vec![vec![1f32, 0f32],
                                      vec![0f32, 1f32]],
                                vec![vec![1f32, 0f32],
                                      vec![0f32, 1f32]]));
  println!("naive: {:?}", naive(vec![vec![2f32, 3f32],
                                      vec![0f32, 1f32]],
                                vec![vec![4f32, 0f32],
                                      vec![0f32, 4f32]]));
  println!("naive: {:?}", naive(vec![vec![1f32, 0f32, 0f32],
                                      vec![0f32, 3f32, 0f32],
                                      vec![0f32, 0f32, 1f32]],
                                vec![vec![1f32, 0f32, 0f32],
                                      vec![0f32, 1f32, 0f32],
                                      vec![0f32, 0f32, 1f32]]));
}