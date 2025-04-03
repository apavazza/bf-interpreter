pub fn run(code: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut memory = vec![0; 30000];
    let mut memory_index = 0;
    let mut code_index = 0;

    loop {
        if code_index >= code.len() {
            break;
        }

        let char = code.chars().nth(code_index).unwrap();

        match char {
            '>' => memory_index = (memory_index + 1) % 256, // Move the index to the right
            '<' => memory_index = (memory_index + 255) % 256, // Move the index to the left
            '+' => memory[memory_index] = (memory[memory_index] + 1) % 255, // Increment the byte at the current index
            '-' => memory[memory_index] = (memory[memory_index] - 1) % 255, // Decrement the byte at the current index
            '.' => print!("{}", memory[memory_index] as char), // Output the byte at the current index as a character
            ',' => {
                // Input a character and store it in memory at the current index
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                memory[memory_index] = input.chars().next().unwrap_or('0') as u8;
            }
            '[' => {
                // Start of a loop
                if memory[memory_index] == 0 {
                    // Skip to the matching ']'
                    let mut depth = 1;
                    while depth > 0 {
                        code_index += 1;
                        if code_index >= code.len() {
                            break;
                        }
                        let next_char = code.chars().nth(code_index).unwrap();
                        if next_char == '[' {
                            depth += 1;
                        } else if next_char == ']' {
                            depth -= 1;
                        }
                    }
                }
            }
            ']' => {
                // End of a loop
                if memory[memory_index] != 0 {
                    // Go back to the matching '['
                    let mut depth = 1;
                    while depth > 0 {
                        code_index -= 1;
                        let prev_char = code.chars().nth(code_index).unwrap();
                        if prev_char == ']' {
                            depth += 1;
                        } else if prev_char == '[' {
                            depth -= 1;
                        }
                    }
                    code_index -= 1; // Because we will increment it again at the end of the loop
                }
            }
            _ => {} // Ignore any other characters
        }

        code_index += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::file;

    use super::*;

    #[test]
    fn test_run() {
        let hello_world = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let mut reader = BufReader::new(hello_world.as_bytes());
        let reader = file::convert(&mut reader).unwrap();
        let result = run(&reader);

        assert!(result.is_ok());
    }
}