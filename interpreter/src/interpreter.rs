use std::cmp::Ordering;
use std::io::Read;

type InstructionPointer = usize;
type DataPointer = usize;

const LIMIT: usize = 30000;
pub struct Interpreter {
    /// index of the next instruction
    pub ip: InstructionPointer,
    /// index of the data[]
    pub dp: DataPointer,
    /// Opening bracket which are not dealt with completely positions.
    opening_brackets: Vec<InstructionPointer>,
    pub data: [u8; LIMIT],
    /// source code
    src: Vec<char>,
}

impl Interpreter {
    pub fn new(src: &str) -> Self {
        Self {
            ip: 0,
            dp: 0,
            opening_brackets: vec![],
            data: [0; LIMIT],
            src: src.chars().collect(),
        }
    }

    /// Get current data cell contents
    pub fn current_cell(&self) -> u8 {
        self.data[self.dp]
    }

    /// Steps one character forward (ignoring whitespace)
    pub fn step(&mut self) -> Result<AtomicResult, Error> {
        match self.ip.cmp(&self.src.len()) {
            Ordering::Equal => Ok(AtomicResult::EndOfProgram),
            Ordering::Greater => Err(Error {
                kind: ErrorKind::IPOutOfBounds,
            }),
            Ordering::Less => {
                let result = self.eval(self.src[self.ip])?;
                if let AtomicResult::Move(ip) = result {
                    self.ip = ip
                }
                self.ip += 1;
                Ok(result)
            }
        }
    }

    /// Evaluates a character in the source.
    fn eval(&mut self, src: char) -> Result<AtomicResult, Error> {
        match src {
            '>' => {
                self.dp = (self.dp.wrapping_add(1)) % LIMIT;
                Ok(AtomicResult::Ok)
            }
            '<' => {
                self.dp = (self.dp.wrapping_sub(1)) % LIMIT;
                Ok(AtomicResult::Ok)
            }
            '+' => {
                self.data[self.dp] = self.data[self.dp].wrapping_add(1);
                Ok(AtomicResult::Ok)
            }
            '-' => {
                self.data[self.dp] = self.data[self.dp].wrapping_sub(1);
                Ok(AtomicResult::Ok)
            }
            '.' => Ok(AtomicResult::Op(self.data[self.dp] as char)),
            ',' => {
                let input: Option<u8> = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok());
                if let Some(input) = input {
                    self.data[self.dp] = input;
                    Ok(AtomicResult::Ok)
                } else {
                    Err(Error {
                        kind: ErrorKind::IOError,
                    })
                }
            }
            '[' => {
                if self.data[self.dp] != 0 {
                    self.opening_brackets.push(self.ip);
                    Ok(AtomicResult::Ok)
                } else {
                    // skip the loop - no change in nesting level

                    let mut i = self.ip + 1;
                    let mut balance = 0;
                    loop {
                        if i >= self.src.len() {
                            break Err(Error {
                                kind: ErrorKind::MissingClosingBracket { ip: self.ip },
                            });
                        }
                        if self.src[i] == ']' {
                            if balance == 0 {
                                // found! this is it.
                                break Ok(AtomicResult::Move(i));
                            } else {
                                balance -= 1;
                            }
                        } else if self.src[i] == '[' {
                            balance += 1;
                        }
                        i += 1;
                    }
                }
            }
            ']' => {
                if self.data[self.dp] != 0 {
                    // go back to the start bracket now
                    if let Some(new_ip) = self.opening_brackets.last().cloned() {
                        Ok(AtomicResult::Move(new_ip))
                    } else {
                        Err(Error {
                            kind: ErrorKind::MissingOpeningBracket(self.ip),
                        })
                    }
                } else if self.opening_brackets.pop().is_none() {
                    Err(Error {
                        kind: ErrorKind::MissingOpeningBracket(self.ip),
                    })
                } else {
                    Ok(AtomicResult::Ok)
                }
            }
            _ => Ok(AtomicResult::Ok),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    IOError,
    MissingClosingBracket { ip: usize },
    MissingOpeningBracket(usize),
    IPOutOfBounds,
}

/// Result of one operation
pub enum AtomicResult {
    Ok,
    EndOfProgram,
    Move(usize),
    Op(char),
}
