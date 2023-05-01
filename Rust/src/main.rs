use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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

fn is_palindrome(x: i32) -> bool {
    if x > 0 || (x % 100 == 0 && x == 0) {
        let stringed: String = x.to_string();
        let reverted: String = stringed.chars().rev().collect::<String>();
        return stringed == reverted;
    }
    return false;
}

fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() {
        return list1;
    }
    if list2.is_none() {
        return list2;
    }
    if list1.unwrap().val <= list2.unwrap().val {
        list1.unwrap().next = merge_two_lists(list1.unwrap().next, list2);
        return list1.clone();
    } else {
        list2.unwrap().next = merge_two_lists(list1, list2.unwrap().next);
        return list2.clone();
    }
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

    let palindrome_number: i32 = 121;
    println!(
        "{:?} {} palindrome",
        palindrome_number,
        if is_palindrome(palindrome_number) {
            "is"
        } else {
            "ís not"
        }
    );
}
