fn main() {
    // Setup
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

    left_list.sort();
    right_list.sort();

    // Part one
    p1(&left_list, &right_list);

    // Part two
    p2(&left_list, &right_list);

    // Results
    // Sum: 2113135
    // Similarity score: 19097157
}

fn p1(left_list: &[i32], right_list: &[i32]) {
    let mut sum = 0;

    if left_list.len() != right_list.len() {
        panic!("Lists are not the same length");
    }

    for i in 0..left_list.len() {
        sum += (left_list[i] - right_list[i]).abs();
    }

    println!("Sum: {}", sum);
}

fn p2(left_list: &[i32], right_list: &[i32]) {
    let mut similarity_score = 0;

    for left_item in left_list {
        similarity_score +=
            right_list.iter().filter(|&x| *x == *left_item).count() as i32 * left_item;
    }

    println!("Similarity score: {}", similarity_score);
}
