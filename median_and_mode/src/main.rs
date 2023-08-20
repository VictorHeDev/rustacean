use std::collections::HashMap;

fn main() {
    let mut list_of_nums = vec![2, 5, 6, 1, 9, 8, 7, 1, 5, 6, 2, 3, 5, 8];
    println!("This is the sorted list: {:?}", list_of_nums);

    println!("The median number is: {}", median(&mut list_of_nums));
    println!("The mode number is: {}", mode(&mut list_of_nums));
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn mode(numbers: &mut [i32]) -> i32 {
    let mut mode_tracker = HashMap::new();

    for num in numbers {
        let count = mode_tracker.entry(num).or_insert(0);
        *count += 1;
    }

    let mut ans = 0;
    let mut curr_max = 0;
    for (key, value) in &mode_tracker {
        if value > &curr_max {
            curr_max = *value;
            ans = **key;
        }
    }
    ans
}
