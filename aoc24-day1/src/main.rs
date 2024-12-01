fn setup() -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    let input = include_str!("input.txt");

    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let left = parts[0].parse::<i32>().unwrap();
        let right = parts[3].parse::<i32>().unwrap();

        left_list.push(left);
        right_list.push(right);
    }

    (left_list, right_list)
}

fn main() {
    // Part one
    let (mut left_list, mut right_list) = setup();

    let mut sum = 0;

    while !left_list.is_empty() && !right_list.is_empty() {
        let left_min_idx = left_list
            .iter()
            .enumerate()
            .min_by_key(|(_, &val)| val)
            .map(|(idx, _)| idx)
            .unwrap();
        let right_min_idx = right_list
            .iter()
            .enumerate()
            .min_by_key(|(_, &val)| val)
            .map(|(idx, _)| idx)
            .unwrap();

        let left_min = left_list.remove(left_min_idx);
        let right_min = right_list.remove(right_min_idx);

        sum += (left_min - right_min).abs();
    }

    println!("Sum: {}", sum);

    // Part two
    let (mut left_list, right_list) = setup();

    let mut similarity_score = 0;

    while !left_list.is_empty() {
        let left_min_idx = left_list
            .iter()
            .enumerate()
            .min_by_key(|(_, &val)| val)
            .map(|(idx, _)| idx)
            .unwrap();

        let left_min = left_list.remove(left_min_idx);
        let right_count = right_list.iter().filter(|&x| *x == left_min).count();

        similarity_score += right_count as i32 * left_min;
    }

    println!("Similarity score: {}", similarity_score);
}
