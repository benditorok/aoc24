fn main() {
    // Setup
    let input = include_str!("input.txt");
    let mut list = vec![];

    for line in input.lines() {
        let parts = line.split(' ');
        let parts: Vec<i32> = parts.map(|x| x.parse::<i32>().unwrap()).collect();

        list.push(parts);
    }

    // Part one
    p1(&list);
    p2(&mut list);
}

fn p1(list: &[Vec<i32>]) {
    let mut safe_count = 0;

    for line in list {
        let ascending = line[0] < line[1];
        let mut valid = true;
        let mut i = 0;
        let end = line.len() - 1;

        if ascending {
            while valid && i < end {
                if line[i] >= line[i + 1] || !valid_difference(line[i], line[i + 1]) {
                    valid = false;
                }

                i += 1;
            }
        } else {
            while valid && i < end {
                if line[i] <= line[i + 1] || !valid_difference(line[i], line[i + 1]) {
                    valid = false;
                }

                i += 1;
            }
        }

        if valid {
            safe_count += 1;
        }
    }

    println!("Safe count: {}", safe_count);
}

fn p2(list: &mut [Vec<i32>]) {
    let mut safe_count = 0;

    for (line_idx, line) in list.iter().enumerate() {
        let ascending = line[0] < line[1];
        let mut valid = true;
        let mut i = 0;
        let end = line.len() - 1;

        if ascending {
            while valid && i < end {
                if line[i] >= line[i + 1] || !valid_difference(line[i], line[i + 1]) {
                    valid = false;
                }

                i += 1;
            }
        } else {
            while valid && i < end {
                if line[i] <= line[i + 1] || !valid_difference(line[i], line[i + 1]) {
                    valid = false;
                }

                i += 1;
            }
        }

        if valid {
            println!("[{}]Valid without removing: {:?}", line_idx, line);
            safe_count += 1;
            continue;
        } else {
            for idx in 0..line.len() - 1 {
                let mut line = line.clone();
                line.remove(idx);

                let ascending = line[0] < line[1];
                let mut valid = true;
                let mut i = 0;
                let end = line.len() - 1;

                if ascending {
                    while valid && i < end {
                        if line[i] >= line[i + 1] || !valid_difference(line[i], line[i + 1]) {
                            valid = false;
                        }

                        i += 1;
                    }
                } else {
                    while valid && i < end {
                        if line[i] <= line[i + 1] || !valid_difference(line[i], line[i + 1]) {
                            valid = false;
                        }

                        i += 1;
                    }
                }

                if valid {
                    safe_count += 1;
                    println!("[{}]Valid after removing: {:?}", line_idx, line);
                    break;
                } else {
                    println!("[{}]Invalid after removing: {:?}", line_idx, line);
                }
            }
        }
    }

    println!("Safe count: {}", safe_count);
}

fn valid_difference(a: i32, b: i32) -> bool {
    (1..=3).contains(&(a - b).abs())
}
