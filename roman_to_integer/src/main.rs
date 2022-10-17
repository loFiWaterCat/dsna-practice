use std::collections::HashMap;

fn main() {
    let test1 = String::from("III");
    let test2 = String::from("LVIII");
    let test3 = String::from("MCMXCIV");
    println!("Expected: 3, Result: {}", roman_to_int(test1));
    println!("Expected: 58, Result: {}", roman_to_int(test2));
    println!("Expected: 1994, Result: {}", roman_to_int(test3));
}

// Convert a roman numeral to integers
fn roman_to_int(s: String) -> i32 {
    let symbol_values = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut sum = 0;
    for (i, c) in s.chars().enumerate() {
        println!("index: {i}, c: {c}, value: {}", symbol_values.get(&c).unwrap());
        println!("sum: {sum}");
        
        // Check the next character if c is not the last character
        if i != s.len() - 1 {
            let next_char = s.chars().nth(i + 1).unwrap();

            if c == 'I' {
                match next_char {
                    'V' => sum -= 2,
                    'X' => sum -= 2,
                    _ => (),
                }
            } else if c == 'X' {
                match next_char {
                    'L' => sum -= 20,
                    'C' => sum -= 20,
                    _ => (),
                }
            } else if c == 'C' {
                match next_char {
                    'D' => sum -= 200,
                    'M' => sum -= 200,
                    _ => (),
                }
            }
        }
            sum += symbol_values.get(&c).unwrap();
    }

    sum
}
