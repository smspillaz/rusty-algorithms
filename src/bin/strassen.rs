/* Strassen's algorithm is another way to multiply two square matrices
 * that is faster than the O(n^3) method in standard matrix multiplication */

/* Matrices are assumed to be in column-major format */
fn naive_sized_accumulate(a: &Vec<Vec<f32>>,
                          a_col_start: usize,
                          a_col_end: usize,
                          a_row_start: usize,
                          a_row_end: usize,
                          b: &Vec<Vec<f32>>,
                          b_col_start: usize,
                          b_col_end: usize,
                          b_row_start: usize,
                          b_row_end: usize,
                          result: &mut Vec<Vec<f32>>) -> u32 {
  let a_m = a_row_end - a_row_start;
  let a_n = a_col_end - a_col_start;
  let b_m = b_row_end - b_row_start;
  let b_n = b_col_end - b_col_start;

  /* Columns in a vs rows in b */
  if a_n != b_m {
    panic!("Incompatible sizes");
  }

  let n = a_m;
  let m = a_n;
  let p = b_n;
  let mut ops = 0;

  for i in 0..m {
    for j in 0..p {
      for k in 0..n {
        ops += 1;
        result[j + a_col_start][i + a_row_start] += a[a_col_start + k][a_row_start + j] * b[b_col_start + i][b_row_start + k];
      }
    }
  }

  ops
}

fn naive(a: Vec<Vec<f32>>, b: Vec<Vec<f32>>) -> (u32, Vec<Vec<f32>>) {
  /* Compatibility check */
  if a.len() != b[0].len() {
    panic!("Can't multiply incompatible matrices");
  }

  let mut result = vec![vec![0.0f32; a.len()]; b[0].len()];

  let ops = naive_sized_accumulate(&a,
                                   0,
                                   a.len(),
                                   0,
                                   a[0].len(),
                                   &b,
                                   0,
                                   b.len(),
                                   0,
                                   b[0].len(),
                                   &mut result);
  return (ops, result)
}

fn naive_sized_accumulate_quadrant(a: &Vec<Vec<f32>>,
                                   a_col: usize,
                                   a_row: usize,
                                   b: &Vec<Vec<f32>>,
                                   b_col: usize,
                                   b_row: usize,
                                   mut result: &mut Vec<Vec<f32>>,
                                   half_n: usize) -> u32 {
  naive_sized_accumulate(&a,
                         half_n * a_col,
                         half_n * (a_col + 1),
                         half_n * a_row,
                         half_n * (a_row + 1),
                         &b,
                         half_n * b_col,
                         half_n * (b_col + 1),
                         half_n * b_row,
                         half_n * (b_row + 1),
                         &mut result)
}

fn nearestpow2(a: u32) -> u32 {
  let mut n = a - 1;

  n = n | (n >> 1);
  n = n | (n >> 2);
  n = n | (n >> 4);
  n = n | (n >> 8);
  n = n | (n >> 16);

  return n + 1;
}

fn copy_into_square(vec: Vec<Vec<f32>>, size: usize) -> Vec<Vec<f32>> {
  let mut result = vec![vec![0f32; size]; size];

  for i in 0..vec.len() {
    for j in 0..vec[0].len() {
      result[i][j] = vec[i][j];
    }
  }

  result
}

fn divideconquer(a_orig: Vec<Vec<f32>>, b_orig: Vec<Vec<f32>>) -> (u32, Vec<Vec<f32>>) {
  /* Compatibility check */
  if a_orig[0].len() != a_orig.len() || b_orig[0].len() != b_orig.len() {
    panic!("Matrices must be square");
  }

  let n = nearestpow2 (a_orig.len() as u32) as usize;
  let half_n = n / 2;

  let mut result = vec![vec![0f32; n]; n];
  let a = copy_into_square(a_orig, n);
  let b = copy_into_square(b_orig, n);

  let mut ops = 0;

  /*
   * [a[0][0] * b[0][0] + a[1][0] * b[0][1], a[0][0] * b[0][1] + a[1][0] * b[1][1],
   *  a[0][1] * b[0][0] + a[1][1] * b[0][1], a[0][1] * b[1][0] + a[1][1] * b[1][1]]
   */

  /* Quadrant 1 */
  ops += naive_sized_accumulate_quadrant(&a, 0, 0,
                                         &b, 0, 0,
                                         &mut result, half_n);
  ops += naive_sized_accumulate_quadrant(&a, 1, 0,
                                         &b, 0, 1,
                                         &mut result, half_n);

  /* Quadrant 2 */
  ops += naive_sized_accumulate_quadrant(&a, 0, 0,
                                         &b, 0, 1,
                                         &mut result, half_n);
  ops += naive_sized_accumulate_quadrant(&a, 1, 0,
                                         &b, 1, 1,
                                         &mut result, half_n);

  /* Quadrant 3 */
  ops += naive_sized_accumulate_quadrant(&a, 0, 1,
                                         &b, 0, 0,
                                         &mut result, half_n);
  ops += naive_sized_accumulate_quadrant(&a, 1, 1,
                                         &b, 0, 1,
                                         &mut result, half_n);

  /* Quadrant 4 */
  ops += naive_sized_accumulate_quadrant(&a, 0, 1,
                                         &b, 1, 0,
                                         &mut result, half_n);
  ops += naive_sized_accumulate_quadrant(&a, 1, 1,
                                         &b, 1, 1,
                                         &mut result, half_n);

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

  println!("divideconquer: {:?}", divideconquer(vec![vec![1f32, 0f32, 0f32],
                                                     vec![0f32, 3f32, 0f32],
                                                     vec![0f32, 0f32, 1f32]],
                                                vec![vec![1f32, 0f32, 0f32],
                                                     vec![0f32, 1f32, 0f32],
                                                     vec![0f32, 0f32, 1f32]]));
}