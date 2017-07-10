fn move_disc(mut from: &mut Vec<i32>, mut to: &mut Vec<i32>, from_tag: &'static str, to_tag: &'static str) {
    println!("Move {:?} ({:?}) to {:?} {:?} {:?}", from_tag, from[from.len() - 1], to_tag, from, to);
    match from.pop() {
        Some(x) => to.push(x),
        _ => panic!("Must have something to move from")
    }
}

fn legal(from: &Vec<i32>, to: &Vec<i32>) -> bool {
    if from.len() > 0 && (to.len() == 0 || from[from.len() - 1] < to[to.len() - 1]) {
        return true;
    }

    return false;
}

fn hanoi(mut left: &mut Vec<i32>, mut mid: &mut Vec<i32>, mut right: &mut Vec<i32>) {
    let len = left.len() as u32;
    let moves = 2i32.pow(len) - 1;

    for i in 1..(moves + 1) {
        if i % 3 == 1 {
            /* Make legal move between left and right */
            if legal(left, right) {
                move_disc(left, right, "left", "right");
            } else if legal(right, left) {
                move_disc(right, left, "right", "left");
            }
        }

        if i % 3 == 2 {
            /* Make legal move between left and mid */
            if legal(left, mid) {
                move_disc(left, mid, "left", "mid");
            } else if legal(mid, left) {
                move_disc(mid, left, "mid", "left");
            }
        }

        if i % 3 == 0 {
            /* Make legal move between mid and right */
            if legal(mid, right) {
                move_disc(mid, right, "mid", "right");
            } else if legal(right, mid) {
                move_disc(right, mid, "right", "mid");
            }
        }
    }
}

fn main() {
    let mut left = vec![5, 4, 3, 2, 1];
    let mut mid = vec![];
    let mut right = vec![];

    hanoi(&mut left, &mut mid, &mut right);
    println!("{:?}", right);
}