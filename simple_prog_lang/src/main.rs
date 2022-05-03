use std::env;

#[derive(Debug, PartialEq)]
enum EvalError {
    UnknownCharacter,
    ZeroDivision,
    InvalidProgram,
}

fn eval(program: &str, index: &mut usize) -> Result<i32, EvalError> {
    let mut chrs = program.chars().skip(*index);
    if let Some(c) = chrs.next() {
        *index += 1;
        match c {
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
            ' ' => eval(&program, index),
            '+' => Ok(eval(&program, index)? + eval(&program, index)?),
            '-' => Ok(eval(&program, index)? - eval(&program, index)?),
            '*' => Ok(eval(&program, index)? * eval(&program, index)?),
            '/' => {
                let lhs = eval(&program, index)?;
                let rhs = eval(&program, index)?;
                if rhs == 0 {
                    Err(EvalError::ZeroDivision)
                } else {
                    Ok(lhs / rhs)
                }
            }
            _ => Err(EvalError::UnknownCharacter),
        }
    } else {
        Err(EvalError::InvalidProgram)
    }
}

fn main() {
    match env::args().nth(1) {
        Some(program) => {
            println!("{} = {}", program, eval(&program, &mut 0).ok().unwrap());
        }
        _ => {
            eprintln!("[USAGE] {} PROGRAM", env::args().nth(0).unwrap());
        }
    }
}

#[test]
fn test_numbers() {
    assert_eq!(Ok(0), eval("0", &mut 0));
    assert_eq!(Ok(1), eval("1", &mut 0));
    assert_eq!(Ok(123), eval("123", &mut 0));
}

#[test]
fn test_add() {
    assert_eq!(Ok(2), eval("+ 1 1", &mut 0));
    assert_eq!(Ok(3), eval("+ 1 2", &mut 0));
    assert_eq!(Ok(579), eval("+ 123 456", &mut 0));
    assert_eq!(Ok(1000), eval("+ 999 1", &mut 0));
}

#[test]
fn test_sub() {
    assert_eq!(Ok(0), eval("- 1 1", &mut 0));
    assert_eq!(Ok(-1), eval("- 1 2", &mut 0));
    assert_eq!(Ok(-333), eval("- 123 456", &mut 0));
    assert_eq!(Ok(998), eval("- 999 1", &mut 0));
}

#[test]
fn test_mul() {
    assert_eq!(Ok(1), eval("* 1 1", &mut 0));
    assert_eq!(Ok(2), eval("* 1 2", &mut 0));
    assert_eq!(Ok(492), eval("* 123 4", &mut 0));
    assert_eq!(Ok(999), eval("* 999 1", &mut 0));
}

#[test]
fn test_div() {
    assert_eq!(Ok(1), eval("/ 1 1", &mut 0));
    assert_eq!(Ok(0), eval("/ 1 2", &mut 0));
    assert_eq!(Ok(2), eval("/ 246 123", &mut 0));
    assert_eq!(Err(EvalError::ZeroDivision), eval("/ 999 0", &mut 0));
}

#[test]
fn test_unknown_character() {
    assert_eq!(Err(EvalError::UnknownCharacter), eval("?", &mut 0));
}

#[test]
fn test_invalid_program() {
    assert_eq!(Err(EvalError::InvalidProgram), eval("", &mut 0));
}
