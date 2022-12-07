use std::io::Read;

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
        #[cfg(debug_assertions)]
        println!(
            "Instruction pointer: {instptr} ({}), Stack pointer: {ptr} ({})",
            input[instptr], stack[ptr as usize]
        );
        match input[instptr] {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => stack[ptr as usize] += 1,
            '-' => stack[ptr as usize] -= 1,
            '.' => printchar(&stack, ptr as usize),
            ',' => getchar(&mut stack, ptr as usize),
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
                if stack[ptr as usize] != 0 {
                    instptr -= 1;
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
    println!();
}

fn printchar(stack: &[u8], ptr: usize) {
    print!("{}", char::from(stack[ptr]));
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

fn getchar(stack: &mut [u8], ptr: usize) {
    loop {
        if let Some(next) = std::io::stdin().bytes().next().and_then(|res| res.ok()) {
            stack[ptr] = next;
            break;
        }
    }
}
