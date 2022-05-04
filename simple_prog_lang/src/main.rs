use std::env;

#[derive(Debug, PartialEq)]
enum EvalError {
    UnknownCharacter,
    ZeroDivision,
    InvalidProgram,
}

struct Parser {
    map: std::collections::HashMap<char, String>,
}
impl Parser {
    pub fn new() -> Parser {
        Parser {
            map: std::collections::HashMap::new(),
        }
    }
    pub fn eval(
        &mut self,
        program: &str,
        index: &mut usize,
        args: Vec<i32>,
    ) -> Result<i32, EvalError> {
        let mut chrs = program.chars().skip(*index);
        if let Some(c) = chrs.next() {
            *index += 1;
            match c {
                // function
                'A'..='Z' => {
                    if let Some(next) = chrs.next() {
                        match next {
                            // function definition
                            '[' => {
                                *index += 1;
                                let last = *index + program[*index..].find(']').unwrap();
                                self.map.insert(c, program[*index..last].to_string());
                                self.eval(program, &mut (last + 1), args)
                            }
                            // function call
                            '(' => {
                                *index += 1;
                                let mut new_args = vec![];
                                let last = *index + program[*index..].find(')').unwrap();
                                while *index < last {
                                    new_args.push(self.eval(program, index, args.clone())?);
                                }
                                if c == 'P' {
                                    // built-in function
                                    println!("{:?}", new_args[0]);
                                    self.eval(program, &mut (last + 1), args)
                                } else {
                                    let p = self.map.get(&c).unwrap().clone();
                                    self.eval(p.as_str(), &mut 0, new_args)
                                }
                            }
                            _ => Err(EvalError::InvalidProgram),
                        }
                    } else {
                        Err(EvalError::InvalidProgram)
                    }
                }

                // function arg
                'a'..='z' => Ok(args[c as usize - 'a' as usize]),

                // number
                '0'..='9' => {
                    let mut val = c as i32 - '0' as i32;
                    while let Some(c) = chrs.next() {
                        if c.is_digit(10) {
                            *index += 1;
                            val = val * 10 + (c as i32 - '0' as i32);
                        } else {
                            break;
                        }
                    }
                    Ok(val)
                }
                // skip spaces
                ' ' => {
                    while let Some(c) = chrs.next() {
                        if c.is_whitespace() {
                            *index += 1;
                        } else {
                            break;
                        }
                    }
                    self.eval(program, index, args)
                }
                // operators
                '+' | '-' | '*' | '/' => {
                    let lhs = self.eval(program, index, args.clone())?;
                    let rhs = self.eval(program, index, args)?;
                    match c {
                        '+' => Ok(lhs + rhs),
                        '-' => Ok(lhs - rhs),
                        '*' => Ok(lhs * rhs),
                        '/' => {
                            if rhs == 0 {
                                Err(EvalError::ZeroDivision)
                            } else {
                                Ok(lhs / rhs)
                            }
                        }
                        _ => Err(EvalError::UnknownCharacter),
                    }
                }
                _ => Err(EvalError::UnknownCharacter),
            }
        } else {
            Err(EvalError::InvalidProgram)
        }
    }
}
fn main() {
    match env::args().nth(1) {
        Some(program) => {
            println!(
                "{} = {}",
                program,
                Parser::new().eval(&program, &mut 0, vec![]).ok().unwrap()
            );
        }
        _ => {
            eprintln!("[USAGE] {} PROGRAM", env::args().nth(0).unwrap());
        }
    }
}

#[test]
fn test_numbers() {
    assert_eq!(Ok(0), Parser::new().eval("0", &mut 0, vec![]));
    assert_eq!(Ok(1), Parser::new().eval("1", &mut 0, vec![]));
    assert_eq!(Ok(123), Parser::new().eval("123", &mut 0, vec![]));
}

#[test]
fn test_add() {
    assert_eq!(Ok(2), Parser::new().eval("+ 1 1", &mut 0, vec![]));
    assert_eq!(Ok(2), Parser::new().eval("+ 1   1", &mut 0, vec![]));
    assert_eq!(Ok(3), Parser::new().eval("+ 1 2", &mut 0, vec![]));
    assert_eq!(Ok(579), Parser::new().eval("+ 123 456", &mut 0, vec![]));
    assert_eq!(Ok(1000), Parser::new().eval("+ 999 1", &mut 0, vec![]));
}

#[test]
fn test_sub() {
    assert_eq!(Ok(0), Parser::new().eval("- 1 1", &mut 0, vec![]));
    assert_eq!(Ok(-1), Parser::new().eval("- 1 2", &mut 0, vec![]));
    assert_eq!(Ok(-333), Parser::new().eval("- 123 456", &mut 0, vec![]));
    assert_eq!(Ok(998), Parser::new().eval("- 999 1", &mut 0, vec![]));
}

#[test]
fn test_mul() {
    assert_eq!(Ok(1), Parser::new().eval("* 1 1", &mut 0, vec![]));
    assert_eq!(Ok(2), Parser::new().eval("* 1 2", &mut 0, vec![]));
    assert_eq!(Ok(492), Parser::new().eval("* 123 4", &mut 0, vec![]));
    assert_eq!(Ok(999), Parser::new().eval("* 999 1", &mut 0, vec![]));
}

#[test]
fn test_div() {
    assert_eq!(Ok(1), Parser::new().eval("/ 1 1", &mut 0, vec![]));
    assert_eq!(Ok(0), Parser::new().eval("/ 1 2", &mut 0, vec![]));
    assert_eq!(Ok(2), Parser::new().eval("/ 246 123", &mut 0, vec![]));
    assert_eq!(
        Err(EvalError::ZeroDivision),
        Parser::new().eval("/ 999 0", &mut 0, vec![])
    );
}

#[test]
fn test_multi_operator() {
    assert_eq!(Ok(10), Parser::new().eval("+ + + 1 2 3 4", &mut 0, vec![]));
    assert_eq!(Ok(14), Parser::new().eval("+ 2 *4 3", &mut 0, vec![]));
    assert_eq!(Ok(33), Parser::new().eval("+ 21* 4 3", &mut 0, vec![]));
}

#[test]
fn test_function() {
    assert_eq!(Ok(2), Parser::new().eval("+ a a", &mut 0, vec![1]));
    assert_eq!(Ok(2), Parser::new().eval("F[+ a a] F(1)", &mut 0, vec![]));
    assert_eq!(Ok(10), Parser::new().eval("F[* a 2] F(5)", &mut 0, vec![]));
    assert_eq!(
        Ok(16),
        Parser::new().eval("F[* a a] F(F(2))", &mut 0, vec![])
    );
    assert_eq!(
        Ok(256),
        Parser::new().eval("F[* a a] F(F(F(2)))", &mut 0, vec![])
    );
}

#[test]
fn test_multi_function() {
    assert_eq!(
        Ok(7),
        Parser::new().eval("F[* a a] G[+ a 3] G(F(2))", &mut 0, vec![])
    );
    assert_eq!(
        Ok(0),
        Parser::new().eval("F[- a 5] G[F(+ a 5)] G(0)", &mut 0, vec![])
    );
}

#[test]
fn test_builtin_function() {
    assert_eq!(
        Ok(12),
        Parser::new().eval("F[* a a] G[+ a 3] P(2) G(F(3))", &mut 0, vec![])
    );
}

#[test]
fn test_multi_arg_function() {
    assert_eq!(Ok(5), Parser::new().eval("F[+ a b] F(2 3)", &mut 0, vec![]));
}

#[test]
fn test_unknown_character() {
    assert_eq!(
        Err(EvalError::UnknownCharacter),
        Parser::new().eval("?", &mut 0, vec![])
    );
}

#[test]
fn test_invalid_program() {
    assert_eq!(
        Err(EvalError::InvalidProgram),
        Parser::new().eval("", &mut 0, vec![])
    );
}
