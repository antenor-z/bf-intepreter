use std::{env, fs::File, io::Read, path::Path, vec};

fn main() {
    let mut instruction_pointer: usize = 0;
    let mut data_pointer: usize = 0;
    const MEMORY_SIZE: usize = 30000;
    let mut memory: Vec<u8> = vec![0; MEMORY_SIZE];

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("supply the file with the source");
    }
    
    let path = Path::new(&args[1]);
    let mut file = match File::open(&path) {
        Err(why) => {panic!("error while opening file. {why}")}
        Ok(file) => {file}
    };

    let mut program_str = String::new();
    match file.read_to_string(&mut program_str) {
        Err(why) => {panic!("error while reading file. {why}")}
        Ok(file_content) => file_content
    };

    let program_memory: Vec<char> = program_str.chars().collect();

    while instruction_pointer < program_memory.len() {
        let instruction: char = program_memory[instruction_pointer];

        if instruction == '>' {
            // Increment the data pointer by one (to point to the next cell to the right). 
            data_pointer += 1;
            if data_pointer > memory.len() {
                panic!("PANIC access after {} bytes ", memory.len());
            }
        }

        else if instruction == '<' {
            // Decrement the data pointer by one (to point to the next cell to the left). 
            data_pointer -= 1;
            if data_pointer > memory.len() {
                panic!("PANIC memory access at less than zero addr");
            }
        }

        else if instruction == '+' {
            // Increment the byte at the data pointer by one. 
            memory[data_pointer] += 1;
        }

        else if instruction == '-' {
            // Decrement the byte at the data pointer by one. 
            memory[data_pointer] -= 1;
        }

        else if instruction == '.' {     
            // Output the byte at the data pointer. 
            print!("{}", memory[data_pointer] as u8 as char);
        }

        else if instruction == ',' {
            let buf: &mut Vec<u8> = &mut vec![0; 1];
            let _ = std::io::stdin().read(buf);
            memory[data_pointer] = buf[0];
        }

        else if instruction == '[' {
            // If the byte at the data pointer is zero, then instead of moving the 
            // instruction pointer forward to the next command, jump it forward to the 
            // command after the matching ] command. 
            if memory[data_pointer] == 0 {
                let mut nesting_count: i32 = 0;
                loop {
                    if program_memory[instruction_pointer] == ']' {
                        nesting_count -= 1;
                    }
                    if program_memory[instruction_pointer] == '[' {
                        nesting_count += 1;
                    }

                    if nesting_count == 0 {
                        break;
                    }
                    instruction_pointer += 1;
                }
            }
        }
        else if instruction == ']' {
            // If the byte at the data pointer is nonzero, then instead of moving the 
            // instruction pointer forward to the next command, jump it back to the 
            // command after the matching [ command.
            if memory[data_pointer] != 0 {
                let mut nesting_count: i32 = 0;
                loop {
                    if program_memory[instruction_pointer] == '[' {
                        nesting_count -= 1;
                    }
                    if program_memory[instruction_pointer] == ']' {
                        nesting_count += 1;
                    }

                    if nesting_count == 0 {
                        break;
                    }
                    instruction_pointer -= 1;
                }
            }
        }


        instruction_pointer += 1;
    }

}
