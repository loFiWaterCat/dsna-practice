fn main() {
    let case1 = Vec::from([2, 1, 2]);
    println!("Case 1 result: {}", largest_perimeter(case1));

    let case2 = Vec::from([1, 2, 1]);
    println!("Case 2 result: {}", largest_perimeter(case2));

    let case3 = vec![3,2,3,4];
    println!("Case 3 expected: 10, result: {}", largest_perimeter(case3))
}

// Takes in a vector of numbers and returns the largest possible perimeter of a triangle or 0 if it
// cannot form a triangle
fn largest_perimeter(nums: Vec<i32>) -> i32 {
    let sum = nums.iter().sum();
    let mut small_sum: i32 = 0;
    let mut largest: i32 = -1;
    for num in nums.iter() {
        if num > &largest {
            largest = *num;
        } else {
            small_sum += num;
        }
    }

    // If the two sides are not larger than the largest side, return 0
    if small_sum < largest {
        return 0
    }

    sum
}
