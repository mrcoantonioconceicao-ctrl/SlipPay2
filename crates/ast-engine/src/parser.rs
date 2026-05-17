#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(char),
    UnexpectedEnd,
    InvalidNumber,
}

pub fn parse_expression(input: &str) -> Result<f64, ParseError> {
    let tokens: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    let mut i = 0;

    fn parse_term(tokens: &[char], i: &mut usize) -> Result<f64, ParseError> {
        let mut value = parse_factor(tokens, i)?;
        while *i < tokens.len() {
            match tokens[*i] {
                '*' => { *i += 1; value *= parse_factor(tokens, i)?; }
                '/' => { *i += 1; value /= parse_factor(tokens, i)?; }
                _ => break,
            }
        }
        Ok(value)
    }

    fn parse_expr(tokens: &[char], i: &mut usize) -> Result<f64, ParseError> {
        let mut value = parse_term(tokens, i)?;
        while *i < tokens.len() {
            match tokens[*i] {
                '+' => { *i += 1; value += parse_term(tokens, i)?; }
                '-' => { *i += 1; value -= parse_term(tokens, i)?; }
                _ => break,
            }
        }
        Ok(value)
    }

    fn parse_factor(tokens: &[char], i: &mut usize) -> Result<f64, ParseError> {
        if *i >= tokens.len() {
            return Err(ParseError::UnexpectedEnd);
        }

        if tokens[*i] == '(' {
            *i += 1;
            let val = parse_expr(tokens, i)?;
            if *i >= tokens.len() || tokens[*i] != ')' {
                return Err(ParseError::UnexpectedChar(')'));
            }
            *i += 1;
            Ok(val)
        } else if tokens[*i].is_ascii_digit() {
            let mut num = String::new();
            while *i < tokens.len() && (tokens[*i].is_ascii_digit() || tokens[*i] == '.') {
                num.push(tokens[*i]);
                *i += 1;
            }
            num.parse::<f64>().map_err(|_| ParseError::InvalidNumber)
        } else {
            Err(ParseError::UnexpectedChar(tokens[*i]))
        }
    }

    parse_expr(&tokens, &mut i)
}
