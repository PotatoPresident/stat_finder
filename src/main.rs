use std::io;
use std::collections::HashMap;

fn main() {
    'logic: loop {
        println!("Enter data set");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim().is_empty() {
            continue;
        }

        let mut data_set = Vec::new();

        for word in input.split_whitespace() {
            let word: i32 = match word.trim().parse() {
                Ok(num) => num,
                Err(_) => continue 'logic
            };
            data_set.push(word);
        }

        println!("Mean: {}", average(&data_set));
        println!("Median: {}", median(&mut data_set));
        println!("Mode: {}", mode(&data_set));
        println!("Range: {}", range(&data_set));
        println!("MAD: {}", mad(&data_set));
        println!("Standard Deviation: {}", std_dev(&data_set));
        println!();
    }
}

fn average(nums: &Vec<i32>) -> f32 {
    let sum: i32 = nums.iter().sum();

    sum as f32 / nums.len() as f32
}

fn median(nums: &mut Vec<i32>) -> f32 {
    nums.sort();

    let mid = nums.len() / 2;

    if nums.len() % 2 == 0 {
        (nums[mid] + nums[mid - 1]) as f32 / 2 as f32
    } else {
        nums[mid] as f32
    }
}

fn mode(nums: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    for &num in nums {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _) | val)
        .expect("Invalid mode")
}

fn range(nums: &Vec<i32>) -> i32 {
    *nums.iter().max().expect("Failed to find max") - *nums.iter().min().expect("Failed to find min")
}

fn mad(nums: &Vec<i32>) -> f32 {
    let mut deviations = Vec::new();
    let mean = average(nums);

    for &num in nums {
        deviations.push((num as f32 - mean).abs())
    }

    deviations.iter().sum::<f32>() / deviations.len() as f32
}

fn std_dev(nums: &Vec<i32>) -> f32 {
    let mut deviations = Vec::new();
    let mean = average(nums);

    for &num in nums {
        deviations.push((num as f32 - mean).powi(2))
    }

    (deviations.iter().sum::<f32>() / deviations.len() as f32).sqrt()
}