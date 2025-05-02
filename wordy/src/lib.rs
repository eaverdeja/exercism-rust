#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    Pow(u32),
}

pub fn answer(command: &str) -> Option<i32> {
    let expression = command.strip_prefix("What is")?.strip_suffix("?")?;
    let tokens = tokenize(expression)?;
    let result = evaluate(tokens)?;

    Some(result)
}

fn tokenize(command: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut iter = command.split_whitespace().peekable();

    while let Some(word) = iter.next() {
        tokens.push(match word {
            "plus" => Token::Plus,
            "minus" => Token::Minus,
            "multiplied" => {
                iter.next_if_eq(&"by")?;
                Token::Multiply
            }
            "divided" => {
                iter.next_if_eq(&"by")?;
                Token::Divide
            }
            "raised" => {
                iter.next_if_eq(&"to")?;
                iter.next_if_eq(&"the")?;
                let power = iter.next().expect("unterminated exponential expression!");
                let token = match power[..power.len() - 2].parse::<u32>() {
                    Ok(num) => Token::Pow(num),
                    Err(_) => return None,
                };
                iter.next_if_eq(&"power")?;

                token
            }
            _ => {
                // The only thing left are numbers
                let sanitized = word.replace(",", "");
                match sanitized.parse::<i32>() {
                    Ok(num) => Token::Number(num),
                    Err(_) => return None, // Everything else is invalid
                }
            }
        })
    }

    Some(tokens)
}

fn evaluate(tokens: Vec<Token>) -> Option<i32> {
    let mut iter = tokens.into_iter();
    let first_token = iter.next()?;

    let first_num = match first_token {
        Token::Number(num) => num,
        _ => return None, // We must start an expression with a number
    };

    let mut result = first_num;
    let mut operator = None;

    for token in iter {
        match token {
            Token::Number(num) => match operator.take() {
                Some(Token::Plus) => result += num,
                Some(Token::Minus) => result -= num,
                Some(Token::Multiply) => result *= num,
                Some(Token::Divide) => result /= num,
                _ => return None, // Invalid operators are not allowed
            },
            Token::Pow(exp) => result = result.pow(exp),
            op @ (Token::Plus | Token::Minus | Token::Multiply | Token::Divide)
                // Consecutive operators are not allowed
                if operator.is_none() => operator = Some(op),
            _ => return None, // Invalid tokens are not allowed
        }
    }

    if operator.is_some() {
        // This means we have a dangling binary operator which
        // was not consumed by matching against a number
        return None;
    }

    Some(result)
}
