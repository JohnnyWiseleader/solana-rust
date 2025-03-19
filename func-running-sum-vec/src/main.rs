fn running_sum(nums: &Vec<i32>) -> Vec<i32> {  
    nums.iter()
        .scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        })
        .collect()
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let result = running_sum(&nums);
    println!("Input: {:?}", nums); 
    println!("Output: {:?}", result);
}