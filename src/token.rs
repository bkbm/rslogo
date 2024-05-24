#[derive(Copy, Clone)]
pub enum TokenType {
    //single character tokens
    Plus,
    Minus,
    Star,
    Slash,
    // few character tokens

    // literals
    Value,
    Identifier,

    //keywords
    PenUp,
    PenDown,
    Forward,
    Back,
    Left,
    Right,
    SetPenColour,
    Turn,
    SetHeading,
    SetX,
    SetY,
    Make,
    AddAssign,
    XCor,
    YCor,
    Heading,
    Colour,
    If,
    While,
    Eq,
    Ne,
    Gt,
    Lt,
    And,
    Or,
}
impl TokenType {
    pub fn to_string(&self) -> String {
        match self {
            TokenType::Plus => String::from("Plus"),
            TokenType::Minus => String::from("Minus"),
            TokenType::Star => String::from("Star"),
            TokenType::Slash => String::from("Slash"),
            TokenType::Value => String::from("Value"),
            TokenType::Identifier => String::from("Identifier"),
            TokenType::PenUp => String::from("PenUp"),
            TokenType::PenDown => String::from("PenDown"),
            TokenType::Forward => String::from("Forward"),
            TokenType::Back => String::from("Back"),
            TokenType::Left => String::from("Left"),
            TokenType::Right => String::from("Right"),
            TokenType::SetPenColour => String::from("SetPenColour"),
            TokenType::Turn => String::from("Turn"),
            TokenType::SetHeading => String::from("SetHeading"),
            TokenType::SetX => String::from("SetX"),
            TokenType::SetY => String::from("SetY"),
            TokenType::Make => String::from("Make"),
            TokenType::AddAssign => String::from("AddAssign"),
            TokenType::XCor => String::from("XCor"),
            TokenType::YCor => String::from("YCor"),
            TokenType::Heading => String::from("Heading"),
            TokenType::Colour => String::from("Colour"),
            TokenType::If => String::from("If"),
            TokenType::While => String::from("While"),
            TokenType::Eq => String::from("Equals"),
            TokenType::Ne => String::from("Not Equals"),
            TokenType::Gt => String::from("Greater Than"),
            TokenType::Lt => String::from("Less Than"),
            TokenType::And => String::from("AND"),
            TokenType::Or => String::from("OR"),
        }
    }
}
pub struct Token {
    lexeme: String,
    token_type: TokenType,
    line: u32,
    column: u32,
}

impl Token {
    pub fn new(lexeme: String, token_type: TokenType, line: u32, column: u32) -> Self {
        Token {
            lexeme,
            token_type,
            line,
            column,
        }
    }
}

impl Token {
    pub fn to_string(&self) -> String {
        format!(
            "[{}: lexeme - '{}', line - {}, column - {}]",
            self.token_type.to_string(),
            self.lexeme,
            self.line,
            self.column
        )
    }
}

#[cfg(test)]
mod test {
    use super::Token;
    use super::TokenType;

    #[test]
    fn token_type_penup_tostring() {
        let ttype = TokenType::PenUp;
        assert_eq!(ttype.to_string(), "PenUp");
    }

    #[test]
    fn token_type_pendown_tostring() {
        let ttype = TokenType::PenDown;
        assert_eq!(ttype.to_string(), "PenDown");
    }

    #[test]
    fn token_type_forward_tostring() {
        let ttype = TokenType::Forward;
        assert_eq!(ttype.to_string(), "Forward");
    }

    #[test]
    fn token_type_back_tostring() {
        let ttype = TokenType::Back;
        assert_eq!(ttype.to_string(), "Back");
    }

    #[test]
    fn token_type_left_tostring() {
        let ttype = TokenType::Left;
        assert_eq!(ttype.to_string(), "Left");
    }

    #[test]
    fn token_type_right_tostring() {
        let ttype = TokenType::Right;
        assert_eq!(ttype.to_string(), "Right");
    }

    #[test]
    fn token_type_setpencolour_tostring() {
        let ttype = TokenType::SetPenColour;
        assert_eq!(ttype.to_string(), "SetPenColour");
    }

    #[test]
    fn token_type_turn_tostring() {
        let ttype = TokenType::Turn;
        assert_eq!(ttype.to_string(), "Turn");
    }

    #[test]
    fn token_type_setheading_tostring() {
        let ttype = TokenType::SetHeading;
        assert_eq!(ttype.to_string(), "SetHeading");
    }

    #[test]
    fn token_type_setx_tostring() {
        let ttype = TokenType::SetX;
        assert_eq!(ttype.to_string(), "SetX");
    }

    #[test]
    fn token_type_sety_tostring() {
        let ttype = TokenType::SetY;
        assert_eq!(ttype.to_string(), "SetY");
    }

    #[test]
    fn token_type_identifier_tostring() {
        let ttype = TokenType::Identifier;
        assert_eq!(ttype.to_string(), "Identifier");
    }

    #[test]
    fn token_type_value_tostring() {
        let ttype = TokenType::Value;
        assert_eq!(ttype.to_string(), "Value");
    }

    #[test]
    fn token_type_plus_tostring() {
        let ttype = TokenType::Plus;
        assert_eq!(ttype.to_string(), "Plus");
    }
    //TODO need to add the remaining tests for new token types
    #[test]
    fn token_tostring() {
        let lexeme = String::from("lexeme");
        let token = Token::new(lexeme.clone(), TokenType::Value, 0, 0);
        assert_eq!(
            token.to_string(),
            String::from("[Value: lexeme - 'lexeme', line - 0, column - 0]")
        );
    }
}
