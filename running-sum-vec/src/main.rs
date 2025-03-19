fn main() {
    let input_nums = vec![1,2,3,4,5];
    let output_nums = running_sum(&input_nums);

    println!("Input: nums = {:?}", input_nums);
    println!("Output: nums = {:?}", output_nums);
}

fn running_sum(nums: &Vec<i32>) -> Vec<i32> {
    let mut running_sum = Vec::with_capacity(nums.len());
    let mut sum = 0;
    
    for num in nums {
        sum += num;
        running_sum.push(sum);
    }
    
    running_sum
}