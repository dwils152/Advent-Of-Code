use std::fs::read_to_string;

fn main() {
    let parsed = get_lines_from_txt();
    let sum = sum_lines(parsed);
    println!("{:?}", sum);
    
}

fn sum_lines(lines: Vec<String>) -> Result<u32, &'static str> {
    let mut total_sum = 0;
    let mut ch_digits_list = Vec::new();
    for line in lines.iter() {
        for c in line.chars() {
            if c.is_ascii_digit() {
                ch_digits_list.push(c);
            }
        }
        let first_digit = ch_digits_list[0].to_digit(10)
            .ok_or("failed to parse digit")?;
        let last_digit = ch_digits_list[ch_digits_list.len()-1]
            .to_digit(10).ok_or("failed to parse digit")?;
        total_sum += 10 * first_digit + last_digit;
        ch_digits_list.clear()
    }
    Ok(total_sum)
}


fn get_lines_from_txt() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}