use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (index_one, &num_one) in nums.iter().enumerate() {
        for (index_two, &num_two) in nums.iter().enumerate() {
            if index_one != index_two && num_one + num_two == target {
                println!(
                    "Numbers: {} - {} Result: {}",
                    num_one,
                    num_two,
                    num_one + num_two
                );
                let addends: Vec<i32> = vec![index_one as i32, index_two as i32];
                println!("Found!");
                return addends;
            }
        }
    }
    return vec![0, 0];
}

fn two_sum_hash_map(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut addends_map = HashMap::new();
    for (index, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if addends_map.contains_key(&complement) {
            return vec![
                index as i32,
                match addends_map.get(&complement) {
                    Some(&number) => number,
                    _ => 0,
                },
            ];
        }
        addends_map.insert(num, index as i32);
    }
    return vec![0, 0];
}

fn main() -> () {
    let numbers: Vec<i32> = (1..101).collect();
    let result: i32 = 58;
    println!("\nNumbers: {:?}\n", numbers);
    println!("Solution: {:?}", two_sum(numbers.clone(), result));
    println!(
        "Solution: {:?}\n",
        two_sum_hash_map(numbers.clone(), result)
    );
}
