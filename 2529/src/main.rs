struct Solution {}

// Solution done by Github: Eveeifyeve 
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let (num_neg, num_pos) = nums.iter().fold((0,0), |(num_neg, num_pos), &x | {(num_neg + (x < 0) as i32, num_pos + (x > 0) as i32)});
        num_neg.max(num_pos)
    }
}

fn main() {
    println!("Positive count: {:?}", Solution::maximum_count(vec![-2,-1,-1,1,2,3]));
    println!("Positive count: {:?}", Solution::maximum_count(vec![-3,-2,-1,0,0,1,2]));
    println!("Positive count: {:?}", Solution::maximum_count(vec![5,  20,  66,  1314]));
}


