use std::collections::HashMap;

fn main() {
    let first_test = TestCase {
        nums: vec!(2, 7, 11, 15),
        target: 9,
    };

    let result1 = first_test.two_sum();
    println!("Result: {:?}", result1);

    let second_test = TestCase::new(vec![3, 2, 4], 6);

    let result2 = second_test.two_sum_one_pass_hash();
    println!("Result: {:?}", result2)
}

struct TestCase {
    nums: Vec<i32>,
    target: i32,
}

impl TestCase {
    pub fn two_sum(&self) -> Vec<i32> {
        let nums = &self.nums;
        let target = self.target;
        // Outer loop
        for (i, num) in nums.iter().enumerate() {
            // Inner loop
            for (j, num2) in nums[i + 1..].iter().enumerate() {
                if num + num2 == target {
                    return vec![i as i32, (j + i + 1) as i32]
                }
            }
        }
        return nums.to_vec();
    }

    pub fn two_sum_one_pass_hash(&self) -> Vec<i32> {
        let mut complements: HashMap<i32, i32> = HashMap::new();
        for (i, num) in self.nums.iter().enumerate() {
            match complements.get(num) {
                Some(&index) => return vec![i as i32, index],
                None => complements.insert(self.target - num, i as i32)
            };
        }

        vec![]
    }

    pub fn new(nums: Vec<i32>, target: i32) -> TestCase {
        TestCase {
            nums,
            target
        }
    }
}
