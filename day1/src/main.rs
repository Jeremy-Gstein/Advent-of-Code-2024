use std::fs::File;
use std::io::{self, BufReader, BufRead};


fn read_lines(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let index: usize = 0;
    let mut day1_answer: i32 = 0;
    for line in reader.lines() {
        let line = line?;
        let mut some_sum: i32 = 0;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let sorted1 = sort_numbers(parts[0]);
            let mut chars: Vec<char> = sorted1.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            chars.reverse();
            let sorted1: String = chars.into_iter().collect();
            
            let sorted2 = sort_numbers(parts[1]);
            let mut chars: Vec<char> = sorted2.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            chars.reverse();
            let sorted2: String = chars.into_iter().collect();

            assert!(sorted1.len() == sorted2.len()); 
            for index in 0..sorted1.len() {
                let num1: char = decode_difference(index, &sorted1);
                let num2: char = decode_difference(index, &sorted2);
                let diff: i32 = (num1 as i32 - num2 as i32).abs();
                some_sum += diff;
                println!("{} - {} = {}", num1, num2, diff);
            }            
            
            println!("Total: {}", some_sum);
            println!("Origin: {} {}", parts[0], parts[1]);
            println!("Sorted: {} {}", sorted1, sorted2);
            //dayday1_answer += some_sum;
            println!("{} += {}", day1_answer, some_sum);
            day1_answer += some_sum;
        }
        //day1_answer += some_sum;
    }
    println!("Finger Crossed: {}", day1_answer);
    Ok(())
}

// take a reference to a string of numbers example "312142" and return a single byte.
fn decode_difference(index: usize, sorted_string: &str) -> char {
    //let sorted_string = sorted_string;
    let bytes = sorted_string.as_bytes();
    if index >= bytes.len() {
        panic!("panic at the byte code");
    }
    let first_byte: u32 = bytes[index] as u32;
    let first_byte_decodes = char::from_u32(first_byte).unwrap();
    //println!("Byte 1: {}", first_byte_decodes);
    return first_byte_decodes;
            
}


fn sort_numbers(numbers: &str) -> String {
    let mut nums: Vec<u32> = numbers.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    nums.sort_unstable();
    nums.iter().map(|&n| n.to_string()).collect::<Vec<_>>().join(" ")
}

fn main() -> io::Result<()> {
    read_lines("input.txt").expect("ERROR");

    Ok(())
}

