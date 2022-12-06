fn main() {
    let input_file = {
        let filename = match std::env::args().nth(1) {
            Some(val) => val,
            None => {
                println!("usage: bf ./program.bf");
                std::process::exit(1);
            }
        };

        std::fs::read_to_string(&filename).expect(&format!("Failed to read file {filename}"))
    };
    let input: Vec<char> = input_file.chars().collect();
    let mut stack = [0u8; 20 as usize];
    let mut ptr = 0u16;
    let mut instptr = 0usize;
    let mut depth = 0;
    while instptr < input.len() {
        match input[instptr] {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => stack[ptr as usize] += 1,
            '-' => stack[ptr as usize] -= 1,
            '.' => writechar(&stack, ptr as usize),
            '[' => {
                if stack[ptr as usize] == 0 {
                    instptr += 1;
                    while input[instptr] != ']' || depth > 0 {
                        if input[instptr] == '[' {
                            depth += 1;
                        } else if input[instptr] == ']' {
                            depth -= 1;
                        }
                        instptr += 1;
                    }
                }
            }
            ']' => {
                if stack[ptr as usize] == 0 {
                    instptr += 1;
                    while input[instptr] != '[' || depth > 0 {
                        if input[instptr] == ']' {
                            depth += 1;
                        } else if input[instptr] == '[' {
                            depth -= 1;
                        }
                        instptr -= 1;
                    }
                }
            }
            _ => {}
        }
        instptr += 1;
    }
}

fn writechar(stack: &[u8], ptr: usize) {
    print!("{}", char::from(stack[ptr]));
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}
