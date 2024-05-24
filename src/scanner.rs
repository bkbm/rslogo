use crate::error::Error;
use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    line: u32,
    column: u32,
    start: u32,
    current: u32,
}

impl Scanner {
    fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            line: 1u32,
            column: 1u32,
            start: 0u32,
            current: 0u32,
        }
    }

    fn is_alphabet(c: char) -> bool {
        match c.to_ascii_lowercase() {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n'
            | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' => true,
            _ => false,
        }
    }
}

impl Scanner {
    fn scan(&mut self) {
        let binding = self.source.clone();
        let mut chars = binding.chars().peekable();
        while let Some(character) = chars.next() {
            self.start = self.current;
            match character {
                //One character lexemes
                '+' => self.add_token(TokenType::Plus, self.substring()),
                '-' => self.add_token(TokenType::Minus, self.substring()),
                '*' => self.add_token(TokenType::Star, self.substring()),
                //Longer Lexemes
                '/' => {
                    if *chars.peek().unwrap() == '/' {
                        chars.next();
                        self.current += 1;
                        while let Some(character) = chars.peek() {
                            if *character == '\n' {
                                break;
                            }
                            chars.next();
                            self.current += 1;
                            self.column += 1;
                        }
                    } else {
                        self.add_token(TokenType::Slash, self.substring())
                    }
                }
                //Whitespace and New line
                ' ' => {}
                '\n' => {
                    self.line += 1;
                    self.column = 0;
                }
                '\"' => {
                    while let Some(character) = chars.peek() {
                        if (character == &' ') | (character == &'\n') {
                            break;
                        }
                        chars.next();
                        self.current += 1;
                        self.column += 1;
                    }
                    self.add_token(TokenType::Value, self.substring())
                }
                ':' => {
                    while let Some(character) = chars.peek() {
                        if (character == &' ') | (character == &'\n') {
                            break;
                        }
                        chars.next();
                        self.current += 1;
                        self.column += 1;
                    }
                    self.add_token(TokenType::Identifier, self.substring())
                }
                other => {
                    if Scanner::is_alphabet(other) {
                        while let Some(character) = chars.peek() {
                            if (character == &' ') | (character == &'\n') {
                                break;
                            }
                            chars.next();
                            self.current += 1;
                            self.column += 1;
                        }

                        let lexeme = self.substring();
                        println!("{}", lexeme);
                        match self.keyword_processing(lexeme) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("implement error reporting manager. btw there is error at line {} and column {}, it says {}", e.get_line(), e.get_column(), e.get_message())
                            }
                        }
                    } else {
                        println!(
                            "change to error system, error at line {} in column {}",
                            self.line, self.column
                        );
                    }
                }
            }
            self.current += 1;
            self.column += 1;
        }
        for token in &self.tokens {
            println!("token: {}", token.to_string());
        }
    }
    fn substring(&self) -> String {
        println!(
            "start: {}, current: {}, len: {}",
            self.start,
            self.current,
            self.source.len()
        );
        String::from(&self.source[(self.start as usize)..=(self.current as usize)])
    }

    fn process_character(&mut self, char_iterator: Vec<u32>) {
        todo!();
    }
    fn keyword_processing(&mut self, lexeme: String) -> Result<(), Error> {
        let mut map = HashMap::new();
        map.insert("PenUp", TokenType::PenUp);
        map.insert("PenDown", TokenType::PenDown);
        map.insert("Forward", TokenType::Forward);
        map.insert("Back", TokenType::Back);
        map.insert("Left", TokenType::Left);
        map.insert("Right", TokenType::Right);
        map.insert("SetPenColour", TokenType::SetPenColour);
        map.insert("Turn", TokenType::Turn);
        map.insert("SetHeading", TokenType::SetHeading);
        map.insert("SetX", TokenType::SetX);
        map.insert("SetY", TokenType::SetY);
        map.insert("Make", TokenType::Make);
        map.insert("AddAssign", TokenType::AddAssign);
        map.insert("XCor", TokenType::XCor);
        map.insert("YCor", TokenType::YCor);
        map.insert("Heading", TokenType::Heading);
        map.insert("Colour", TokenType::Colour);
        map.insert("If", TokenType::If);
        map.insert("While", TokenType::While);
        map.insert("EQ", TokenType::Eq);
        map.insert("NE", TokenType::Ne);
        map.insert("GT", TokenType::Gt);
        map.insert("LT", TokenType::Lt);
        map.insert("AND", TokenType::And);
        map.insert("OR", TokenType::Or);
        match map.get(&lexeme[..]) {
            Some(x) => {
                self.add_token(x.clone(), lexeme);
                Ok(())
            }
            None => Err(Error::new(
                "keyword doesn't exist".to_string(),
                (self.line, self.column),
            )),
        }
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: String) {
        let token = Token::new(lexeme, token_type, self.line, self.column);
        self.tokens.push(token);
    }
}

#[cfg(test)]
pub mod test {
    use super::Scanner;
    #[test]
    fn test1() {
        let input = String::from("PenDown\nForward \"100 ");
        let mut scanner = Scanner::new(input.clone());
        scanner.scan();
        //todo!();
    }

    #[test]
    fn ignores_comments() {
        let input = String::from("PenDown\n//hello world\nForward \"100");
        let mut scanner = Scanner::new(input.clone());
        scanner.scan();
        //println!("{}",scanner.tokens);
        todo!()
    }
}
