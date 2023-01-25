use std::collections::HashMap;

pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        let div = target - num;
        if hashmap.contains_key(&div) {
            return Vec::from([*hashmap.get(&div).unwrap(), index as i32]);
        }
        hashmap.insert(*num, index as i32);
    }

    Vec::new()
}

