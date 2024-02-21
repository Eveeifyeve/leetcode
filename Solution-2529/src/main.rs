struct Solution {}

// Solution done by Github: Eveeifyeve 
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut count =  0; // Defines count
        for value in nums { // For each number in vec = value
            if value >  0 {count +=  1; } // if the value is above 0 return count with +1
        }
        count // Returns count
    }
}

fn main() {
    println!("Pos: {:?}", Solution::maximum_count(vec![21, 23, 1, 70, 50]));
}