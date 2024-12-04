use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

fn main() {}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

struct Mul {
    x: u64,
    y: u64,
}

impl TryFrom<&str> for Mul {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let matches = &['(', ')'];
        let numbers: Vec<_> = value
            .split(',')
            .into_iter()
            .filter_map(|n| {
                if n.is_empty() {
                    return None;
                }

                match n.trim_matches(matches).parse::<u64>() {
                    Ok(i) => Some(i),
                    Err(_e) => None
            }})
            .collect();

        if numbers.len() > 1 {
            if !(1..999).contains(&numbers[0]) || !(1..999).contains(&numbers[1]) {
                return Err(());
            }

            return Ok(Self {
                x: numbers[0],
                y: numbers[1]
            });
        }

        Err(())
    }
}

static REGEX_SOL1: &'static str = r"(mul\({1}([1-9]?[0-9]?\d{1}),([1-9]?[0-9]?\d{1})\){1})+";

fn regex(input: &str, reg: &'static str) -> Vec<Mul> {
    let mut results = vec![];
    let re = Regex::new(reg).unwrap();
    for (full, [capture, x, y]) in re.captures_iter(input).map(|c| c.extract()) {
        if full == capture {
            results.push(Mul {
                x: x.parse::<u64>().unwrap(),
                y: y.parse::<u64>().unwrap()
            });
        } else {
            let tuples = full
                .split("mul")
                .into_iter()
                .filter_map(|e| {
                    if e.is_empty() {
                        return None;
                    }

                    let matches = &['(', ')', '.'];
                    let numbers: Vec<_> = e
                        .split(',')
                        .into_iter()
                        .filter_map(|n| Some(n.trim_matches(matches).parse::<u64>().unwrap()))
                        .collect();

                    Some((numbers[0], numbers[1]))
                })
                .collect::<Vec<_>>();

            for (x, y) in tuples {
                results.push(Mul {x, y});
            }
        }
    }

    results
}

struct Tokenizer {
    source: Vec<char>,
    current_position: usize,
    line: usize,
    column: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    DO,
    DONT,
    MUL,
    Number,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenType,
    literal: String,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(source: Vec<char>) -> Self {
        Self {
            source,
            current_position: 0,
            line: 1,
            column: 1,
        }
    }

    fn current(&mut self) -> Option<&char> {
        self.source.get(self.current_position)
    }

    fn advance(&mut self) {
        self.current_position += 1;
        self.column += 1;
    }

    fn read_mul(&mut self) -> Token {
        let mut action = String::new();
        let line = self.line;
        let column = self.column;
        let mut token = Token { kind: TokenType::Unknown, literal: String::new(), line, column };

        self.advance();
regex(input.as_str(), REGEX_SOL1))
        if self.current().is_some_and(|c| *c == 'u') && self.peek().is_some_and(|c| *c == 'l') {
            for _ in 0..2 {
                self.advance();
            }

            if self.current().is_some_and(|c| *c == '(') && self.peek().is_some_and(|c| c.is_numeric()) {
                self.advance();
            }

            if self.current().is_some_and(|c| c.is_numeric()) {
                action.push_str(self.read_number().literal.as_str());
                if self.current().is_some_and(|c| *c == ',') {
                    action.push(',');
                    self.advance();
                    if self.current().is_some_and(|c| c.is_numeric()) {
                        action.push_str(self.read_number().literal.as_str());
                        if self.current().is_some_and(|c| *c ==')') {
                            self.advance();
                            token.kind = TokenType::MUL;
                        }
                    }
                }

            }
        }

        token.literal = action;regex(input.as_str(), REGEX_SOL1))
        token
    }

    fn read_number(&mut self) -> Token {
        self.create_token(TokenType::Number, |c| c.is_numeric())
    }

    fn read_do(&mut self) -> Token {
        let mut action = String::new();
        // ive checked it before
        for _ in 0..2 {
            action.push(*self.current().unwrap());
            self.advance();
        }
        let line = self.line;
        let column = self.column;

        if self.current().is_some_and(|c| *c == '(') && self.peek().is_some_and(|c| *c == ')') {
            for _ in 0..2 {
                action.push(*self.current().unwrap());
                self.advance();
            }

            return Token { kind: TokenType::DO, literal: action, line, column };
        }

        if self.current().is_some_and(|c| *c == 'n') && self.peek().is_some_and(|c| *c == '\'') {
            for _ in 0..2 {
                action.push(*self.current().unwrap());
                self.advance();
            }

            if self.current().is_some_and(|c| *c == 't') {
                action.push(*self.current().unwrap());
                self.advance();

                if self.current().is_some_and(|c| *c == '(') && self.peek().is_some_and(|c| *c == ')') {
                    for _ in 0..2 {
                        action.push(*self.current().unwrap());
                        self.advance();
                    }
                }

                return Token { kind: TokenType::DONT, literal: action, line, column };
            }


        }

        return Token { kind: TokenType::Unknown, literal: action, line, column };
    }
    
    fn peek(&self) -> Option<&char> {
        self.source.get(self.current_position + 1)
    }

    fn tokenize(&mut self) -> Vec<Mul> {regex(input.as_str(), REGEX_SOL1))
        let mut tokens = vec![];

        loop {
            let cc = self.current();
            match cc {
                None => break,
                Some(c) => {
                    if *c == 'm' {
                        tokens.push(self.read_mul());
                        continue;
                    }

                    if *c == 'd' && self.peek().is_some_and(|o| *o == 'o') {
                        tokens.push(self.read_do());
                        continue;
                    }

                    self.advance();
                }
            }
        }

        let mut dos = true;

        let mut r: Vec<Mul> = vec![];
        tokens = tokens.into_iter().filter(|t| !matches!(t.kind, TokenType::Number | TokenType::Unknown)).collect::<Vec<Token>>();

        for token in tokens {
            match token.kind {
                TokenType::DONT => dos = false,regex(input.as_str(), REGEX_SOL1))
                TokenType::DO => dos = true,
                TokenType::MUL => {
                    if dos {
                        match Mul::try_from(token.literal.as_str()) {
                            Ok(m) => r.push(m),
                            Err(_e) => {}
                        }
                    }
                },
                _ => {}
            }
        }

        r
    }

    fn create_token<F>(&mut self, kind: TokenType, read_fn: F) -> Token
    where
        F: Fn(char) -> bool,
    {
        let line = self.line;
        let column = self.column;
        let literal = self.read_until(read_fn);

        Token { kind, literal, line, column }
    }

    fn read_until<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut str = String::new();
        loop {
            match self.current() {
                None => break,
                Some(c) => {
                    if condition(*c) {
                        str.push(*c);
                        self.advance();
                    } else {
                        break;
                    }
                }
            };
        }

        str
    }
}

fn calculate(vec: Vec<Mul>) -> u64 {
    vec.into_iter().map(|op| op.x * op.y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_case_7() {
        let input = "xmul(2,4)&m()don't()6mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let mut tokenizer = Tokenizer::new(input.chars().collect::<Vec<_>>());
        assert_eq!(48, calculate(tokenizer.tokenize()));
    }

    #[test]
    fn test_case_6() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let mut tokenizer = Tokenizer::new(input.chars().collect::<Vec<_>>());
        assert_eq!(48, calculate(tokenizer.tokenize()));
    }
    

    #[test]
    fn test_case_5() {
        let input = "{[#select()@mul(308,194)!'#*don't();&mul(895,408)?<do(), [(where()]]mul(909,700)?{>";

        let mut tokenizer = Tokenizer::new(input.chars().collect::<Vec<_>>());
        assert_eq!(696052, calculate(tokenizer.tokenize()));
    }


    #[test]
    fn test_case_4() {
        let input = "{[#select()@mul(308,194)!'#*;&mul(895,408)?<, [(where()]]mul(909,700)?{>";

        assert_eq!(1061212, calculate(regex(input, REGEX_SOL1)));
    }
    #[test]
    fn test_case_3() {
        let input = "mul(180,971)...mul(914,983)#who()";

        assert_eq!(1073242, calculate(regex(input, REGEX_SOL1)));
    }
    #[test]
    fn test_case_2() {
        let input = "from(563,273)mul(180,971)(,mul(914,983)#who()";

        assert_eq!(1073242, calculate(regex(input, REGEX_SOL1)));
    }


    #[test]
    fn test_case_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)";

        assert_eq!(161, calculate(regex(input, REGEX_SOL1)));
    }

    #[test]
    fn test_solution1() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_file("input.txt")?;

        assert_eq!(179834255, calculate(regex(input.as_str(), REGEX_SOL1)));

        Ok(())

    }

    #[test]
    fn test_solution2() -> Result<(), Box<dyn std::error::Error>> {
        let input = read_file("input2.txt")?;

        let input = input.chars().into_iter().collect();
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(80570939, calculate(tokenizer.tokenize()));

        Ok(())

    }
}
