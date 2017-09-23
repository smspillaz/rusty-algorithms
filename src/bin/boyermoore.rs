static ASCII_LOWER: &'static [char] = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                                        'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                                        's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

fn find_pending_character_index(chars: &Vec<char>, start: usize, needle: &char) -> usize {
  let len = chars.len();
  for i in (start + 1)..len {
    if chars[i] == *needle {
      return i - start;
    }
  }

  return 0;
}

fn bad_character_table(pattern: &str) -> Vec<Vec<usize>> {
  /* Go backwards through the string for each character and work out how many characters
   * away that character is (backwards) */
  let mut table = vec![vec![(0 as usize); pattern.len()]; ASCII_LOWER.len()];
  let chars: Vec<char> = pattern.chars().rev().collect();
  let len = chars.len();

  for (idx, _) in chars.iter().enumerate() {
    for (alphabet_index, c) in ASCII_LOWER.iter().enumerate() {
      /* How many characters from this one until we reach the end
       * or see this character again */
      let res = find_pending_character_index(&chars, idx, c);
      table[alphabet_index][(len - 1) - idx] = res;
    }
  }

  return table;
}

fn matching_subpatterns(chars: &Vec<char>, left: usize, right: usize, len: usize) -> bool {
  for i in 0..len {
    if chars[left + i] != chars[right + i] {
      return false;
    }
  }

  return true;
}

fn good_suffix_table(pattern: &str) -> Vec<usize> {
  /* For each character in the string, take the subpattern from char[i]..char[len]
   * and see if such a subpattern exists elsewhere in the string, moving the window
   * back one by one. This is an O(N^2) operation over the pattern, since you have
   * to check the subpattern over each window from the current position. */
  let chars: Vec<char> = pattern.chars().rev().collect();
  let mut table = vec![(0 as usize); chars.len()];

  for idx in 0..chars.len() {
    /* Add one here since the outer loop was exclusive, we do not want the inner
     * loop to be exclusive of the last index */
    for candidate in 1..((chars.len() + 1) - idx) {
      /* Checking two empty subpatterns against each other doesn't make
       * much sense - we want a full shift in that case */
      if idx > 0 && matching_subpatterns(&chars, 0, candidate, idx) {
        table[(chars.len() - 1) - idx] = candidate;
        break;
      }
    }
  }

  return table;
}

fn boyer_moore(input: &str, pattern: &str, bad_character: &Vec<Vec<usize>>, good_suffix: &Vec<usize>) -> Option<usize> {
  /* Some obvious cases */
  let pattern_chars: Vec<char> = pattern.chars().collect();
  let input_chars: Vec<char> = input.chars().collect();

  if pattern_chars.len() > input_chars.len() {
    return None;
  }

  let mut alignment = 0;
  let max_alignment = input.len() - pattern.len();

  loop {
    let mut mismatch = false;

    for (pattern_index, i) in (alignment..(alignment + pattern_chars.len())).enumerate().rev() {
      if input_chars[i] != pattern_chars[pattern_index] {
        mismatch = true;

        /* If we're on the last alignment, return now, as there are no matches */
        if alignment == (input.len() - pattern.len()) {
          return None;
        }

        /* Mismatch occurred here, look up this character/index pair
         * in the good suffix and bad character tables to see how far
         * forward to shift. If the shift amount is zero, shift
         * forward by the entire pattern size */
        let charidx = (input_chars[i] as usize) - ('a' as usize);
        let shift = std::cmp::max(bad_character[charidx][pattern_index],
                                  good_suffix[pattern_index]);

        if shift == 0 {
          alignment = std::cmp::min(max_alignment, alignment + pattern_chars.len());
        } else {
          alignment = std::cmp::min(max_alignment, alignment + shift);
        }
        break;
      }
    }

    /* No mismatches, return the current alignment */
    if !mismatch {
      return Some(alignment);
    }
  }
}

fn main() {
  let pattern = "abcab";
  let bad_character = bad_character_table(pattern);
  let good_suffix = good_suffix_table(pattern);
  println!("{:?}", bad_character);
  println!("{:?}", good_suffix);
  println!("{:?}", boyer_moore("bbabbaccabababcabababab", pattern, &bad_character, &good_suffix));
}