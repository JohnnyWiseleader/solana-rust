use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, Vec<Vec<usize>>> = HashMap::new();
    let n = nums.len();

    // Iterate over all unique pairs of indexes (i, j)
    for i in 0..n {
        for j in (i + 1)..n {
            // Ensure i < j to avoid duplicates
            let value_sum = nums[i] + nums[j]; // Sum the values at indexes i and j
            let index_pair = vec![i, j];

            // Insert into the hashmap, grouping pairs by their value sum
            map.entry(value_sum)
                .or_insert_with(Vec::new)
                .push(index_pair);
        }
    }

    // Print debugging info
    println!("Input: nums = {:?}, target = {}", nums, target);

    // Use .get(&target) to avoid panic
    if let Some(pairs) = map.get(&target) {
        if pairs.len() > 1 {
            println!(
                "WARNING: number of pairs {} - returning first pair only.",
                pairs.len()
            );
        }
        if let Some(pair) = pairs.first() {
            return vec![pair[0] as i32, pair[1] as i32];
        }
    }

    // Return empty Vec if no valid pair is found
    vec![]
}

fn main() {
    println!("{:?}", two_sum(vec![2, 3, 4, 5], 7));
}
