fn main() {
    let input = include_str!("../input.txt");

    let max_seat_id = input
        .lines()
        .map(|l| seat_pos(l))
        .map(|(row, col)| row * 8 + col)
        .max()
        .unwrap();
    println!("Max seat ID:{}", max_seat_id);

    let mut filled_seats: Vec<u32> = input
        .lines()
        .map(|l| seat_pos(l))
        .map(|(row, col)| row * 8 + col)
        .collect();
    filled_seats.sort();

    let offset = filled_seats[0] as usize;
    let my_seat = filled_seats
        .into_iter()
        .enumerate()
        .find(|(nr, val)| *val as usize != (*nr + offset))
        .map(|(nr, _)| nr + offset)
        .unwrap();

    println!("My seat: {}", my_seat);
}

// BFFFBBF RRR
fn seat_pos(input: &str) -> (u32, u32) {
    // 7
    let front_back = input.get(0..7).unwrap();
    let (front, _back) = front_back.chars().fold((0, 127), |r, c| split_seats(r, c));

    let left_right = input.get(7..10).unwrap();
    let (left, _right) = left_right.chars().fold((0, 7), |r, c| split_seats(r, c));

    (front, left)
}

// front 0, back 128
// left 0, right 7
fn split_seats((start, end): (u32, u32), c: char) -> (u32, u32) {
    let midpoint = start + (end - start) / 2;
    match c {
        'F' | 'L' => (start, midpoint),
        'B' | 'R' => (midpoint + 1, end),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_test() {
        assert_eq!(split_seats((0, 127), 'F'), (0, 63));
        assert_eq!(split_seats((0, 63), 'B'), (32, 63));
        assert_eq!(split_seats((0, 127), 'F'), (32, 47));
        assert_eq!(split_seats((0, 127), 'B'), (40, 47));
        assert_eq!(split_seats((0, 127), 'B'), (44, 47));
        assert_eq!(split_seats((0, 127), 'F'), (44, 45));
        assert_eq!(split_seats((0, 127), 'F'), (44, 44));
    }
}
