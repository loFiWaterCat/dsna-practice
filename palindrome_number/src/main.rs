// Finished

fn main() {
    let x = -121;
    let y = 10;
    let z = 1000021;
    let a = 1234321;
    let b = 123321;
    // println!("{x}: {}", is_palindrome(x));
    // println!("{y}: {}", is_palindrome(y));
    println!("{b}: {}", is_palindrome(b));
}

// Takes in a number and returns a bool, true if it's a palindrome
fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut current = x.to_string();

    while current.len() > 0 {
        let first_char = &current[0..1];
        let last_char = &current[current.len() - 1..];

        if first_char != last_char {
            return false;
        }

        if (current.len() == 1) {
            return true;
        } else {
            current = current[1..current.len()-1].to_string();
        }
    }

    return true;
}
