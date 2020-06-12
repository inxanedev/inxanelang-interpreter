use std::env;
use std::fs;
fn main() {
    ctrlc::set_handler(move || {
        std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
    	println!("Specify file name as argument!");
    } else {
    	let code: String = match fs::read_to_string(args[1].clone()) {
            Ok(s) => s,
            Err(_) => {
                error(0, "Error reading the file!", false);
                return;
            }
        };
    	let split: Vec<&str> = code.split('\n').map(|x| x.trim()).collect();
    	let mut stack: Vec<i32> = Vec::new();
    	let mut code_line = 0;
    	loop {
    		let split_line: Vec<&str> = split[code_line].split(' ').collect();
    		match split_line[0] {
    			"STOP" => {
    				break;
    			},
    			"PRINT" => {
                    if split_line.len() == 1 {
                        error(code_line, "No arguments to PRINT specified!", true);
                        break;
                    }
    				print!("{}", build_string(&split_line, 1, split_line.len()));
    			},
    			"GOTO" => {
                    if split_line.len() == 1 {
                        error(code_line, "No line specified for GOTO statement!", true);
                        break;
                    }
                    let line = match split_line[1].parse::<usize>() {
                        Ok(l) => l,
                        Err(_) => {
                            error(code_line, "Invalid number passed to GOTO statement!", true);
                            break;
                        }
                    };
    				if line <= split.len() && line > 0 {
                        code_line = line - 1;
    				} else {
                        error(code_line, "Line number out of bounds!", true);
                        break;
                    }
                    continue;
    			},
    			"PUSH" => {
    				stack.push(match split_line[1].parse::<i32>() {
                        Ok(v) => v,
                        Err(_) => {
                            error(code_line, "Invalid number passed to PUSH statement!", true);
                            break;
                        }
                    });
    			},
    			"POP" => {
    				stack.pop();
    			},
    			"ADD" => {
    				*stack.last_mut().unwrap() += match split_line[1].parse::<i32>() {
                        Ok(v) => v,
                        Err(_) => {
                            error(code_line, "Invalid number passed to ADD statement!", true);
                            break;
                        }
                    }
    			},
                "SUB" => {
                    *stack.last_mut().unwrap() -= match split_line[1].parse::<i32>() {
                        Ok(v) => v,
                        Err(_) => {
                            error(code_line, "Invalid number passed to SUB statement!", true);
                            break;
                        }
                    }
                },
                "MULT" => {
                    *stack.last_mut().unwrap() *= match split_line[1].parse::<i32>() {
                        Ok(v) => v,
                        Err(_) => {
                            error(code_line, "Invalid number passed to MULT statement!", true);
                            break;
                        }
                    }
                },
    			"POPPRINT" => {
    				print!("{}", stack.pop().unwrap());
    			},
                "APOPPRINT" => {
                    print!("{}", std::char::from_u32(stack.pop().unwrap() as u32).unwrap());
                },
                "IF" => {
                    let mut value = 0;
                    if stack.len() != 0 {
                        value = stack.pop().unwrap();
                    }
                    if value == match split_line[1].parse::<i32>() {
                        Ok(v) => v,
                        Err(_) => {
                            error(code_line, "Invalid number passed to IF statement!", true);
                            break;
                        }
                    } {
                        code_line += 1;
                        continue;
                    } else {
                        code_line += 2;
                        continue;
                    }
                },
                "PRINTSTACK" => {
                    print!("{:?}", stack);
                },
                "REV" => {
                    stack.reverse();
                },
                "INPUTSTR" => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Failed to read line!");
                    input = input.trim().to_string();
                    let mut input_bytes_i32: Vec<i32> = input.into_bytes().into_iter().map(|x| x as i32).collect();
                    stack.append(&mut input_bytes_i32);
                },
                "INPUTNUM" => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Failed to read line!");
                    input = input.trim().to_string();
                    stack.push(input.parse::<i32>().unwrap());
                },
                "LEN" => {
                    stack.push(stack.len() as i32);
                },
                "DUP" => {
                    let popped = stack.pop().unwrap();
                    stack.push(popped);
                    stack.push(popped);
                },
                "NEWLINE" => {
                    print!("\n");
                },
    			_ => {}
    		};
    		code_line += 1;
    		if code_line == split.len() {
    			break;
    		}
    	}
    }
}

fn build_string(vector: &Vec<&str>, start_index: usize, end_index: usize) -> String {
	let mut output = String::new();
	for i in start_index..end_index {
		output.push_str(vector[i]);
		if i != end_index - 1 {
			output.push_str(" ");
		}
	}
	return output;
}

fn error(line: usize, message: &str, take_line: bool) {
    match take_line {
        true => eprintln!("ERROR (line {}): {}", line + 1, message),
        false => eprintln!("ERROR: {}", message)
    };
}