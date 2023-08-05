/* Usage : pixelxs_to_graphical <path/to/file> */
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage : pixel <path/to/file>");
        return;
    }

    // &args[1] is the path to the file containing the pixels
    let file_content = match fs::read_to_string(&args[1]) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error when reading the file : {}", err);
            return;
        }
    };

    let lines: Vec<&str> = file_content.lines().collect();

    for line in lines {
        let mut graphical_repr = String::new(); // This contains the 'drawing'
        let parts: Vec<&str> = line.split('+').collect();
        for part in parts { // Each mulitplication
            let (digit, count) = parse_part(part);
            if digit == '1' { // If it's black then repeat black rectangle 'count' times
                graphical_repr.push_str(&"█".repeat(count as usize));
            } else {
                graphical_repr.push_str(&" ".repeat(count as usize));
            }
        }
        println!("{}", graphical_repr);
    }
}

fn parse_part(part: &str) -> (char, u32) {
    let parts: Vec<&str> = part.split('x').collect();

    if let [digit_str, count_str] = parts.as_slice() {
        if let (Some(digit), Ok(count)) = (digit_str.chars().next(), count_str.parse::<u32>()) {
            return (digit, count);
        }
    }
    ('0', 0)
}
