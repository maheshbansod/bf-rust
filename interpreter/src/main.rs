mod interpreter;

use std::env;
use std::io::Write;

use interpreter::{AtomicResult, Interpreter};
fn main() {
    let hello_world = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let _test_print_one = "+++++++[>+++++++<-]>.";
    let mut interpreter = Interpreter::new(hello_world);

    let args: Vec<String> = env::args().collect();

    let interactive = args.len() > 1 && args[1] == "i";

    if !interactive {
        // TODO: read a file and use it
        loop {
            match interpreter.step() {
                Ok(res) => match res {
                    AtomicResult::Op(op) => print!("{}", op),
                    AtomicResult::EndOfProgram => break,
                    _ => {}
                },
                Err(err) => {
                    println!("{:?}", err);
                    break;
                }
            }
        }
    } else {
        // interactive here
        // commands -> data, ip, dp, step, run, exit
        // data [i] -> ith entry in data array or data[dp]
        // ip -> instruction pointer
        // dp -> data pointer
        // step [i] -> step i instructions or 1 instruction,
        // run -> run from current step to the end
        // exit -> exit
        println!("A simple BF debugger.");
        loop {
            print!("\n >> ");
            std::io::stdout().flush().unwrap();

            let mut cmd = String::new();
            std::io::stdin().read_line(&mut cmd).unwrap();
            cmd.pop();
            let cmd: Vec<&str> = cmd.split_ascii_whitespace().collect();
            if cmd.len() > 0 {
                match cmd[0] {
                    "data" => {
                        // print data [i] or data [dp]
                        let output =
                            if cmd.len() > 1 {
                                cmd.iter().skip(1).map(|x| match x.parse::<usize>() {
                                Ok(i) => match interpreter.data.get(i) {
                                    Some(data) => format!("DATA[{}]={}", i, data),
                                    None => format!("Index `{}` out of bounds.", i),
                                },
                                Err(_e) => {
                                    format!("Couldn't convert `{}` to a non-negative integer.\n", x)
                                }
                            }).collect::<Vec<String>>().join("\n")
                            } else {
                                format!("DATA[{}]={}", interpreter.dp, interpreter.current_cell())
                            };
                        println!("{}", output);
                    }
                    "ip" => {
                        println!("IP: {}", interpreter.ip);
                    }
                    "dp" => {
                        println!("DP: {}", interpreter.dp);
                    }
                    "step" => {
                        let step_by = if cmd.len() > 1 {
                            if let Ok(step_by) = cmd[1].parse::<usize>() {
                                step_by
                            } else {
                                0
                            }
                        } else {
                            1
                        };
                        if step_by == 0 {
                            println!("The argument to this should be >= 1");
                        } else {
                            println!("Stepping {} instructions.", step_by);
                            for _ in 0..step_by {
                                match interpreter.step() {
                                    Ok(res) => match res {
                                        AtomicResult::Op(op) => print!("{}", op),
                                        AtomicResult::EndOfProgram => break,
                                        _ => {}
                                    },
                                    Err(err) => {
                                        println!("{:?}", err);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    "run" => loop {
                        match interpreter.step() {
                            Ok(res) => match res {
                                AtomicResult::Op(op) => print!("{}", op),
                                AtomicResult::EndOfProgram => break,
                                _ => {}
                            },
                            Err(err) => {
                                println!("{:?}", err);
                                break;
                            }
                        }
                    },
                    "exit" => break,
                    _ => {
                        println!("The command `{}` is not recognised.", cmd[0]);
                    }
                }
            }
        }
    }
}
