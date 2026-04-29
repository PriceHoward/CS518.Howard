use crate::enums::Token;

pub struct Reader {
    input: Vec<char>,
    position: usize,
}

impl Reader {
    pub fn new(source: &str) -> Self {
        Reader {
            input: source.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            self.skip_whitespace();
            if self.position >= self.input.len() {
                break;
            }
            let ch = self.input[self.position];
            let token = match ch {
                '{' => { self.position += 1; Token::LBrace }
                '}' => { self.position += 1; Token::RBrace }
                '-' | '0'..='9' => self.reader_number()?,
                'a'..='z' | 'A'..='Z' => self.reader_word(),
                other => return Err(format!("Unexpected character: '{}'", other)),
            };
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn reader_number(&mut self) -> Result<Token, String> {
        let starting_pos = self.position;

        if self.input[self.position] == '-' {
            self.position += 1;
        }

        while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
            self.position += 1;
        }

        while self.position < self.input.len() && self.input[self.position] == '.' {
            self.position +=1;
            while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
                self.position += 1;
            }
        }
        
        let s: String = self.input[starting_pos..self.position].iter().collect();
        s.parse::<f64>()
            .map(Token::Number)
            .map_err(|_| format!("Invalid number: '{}'", s))
    }

    fn reader_word(&mut self) -> Token {
        let starting_pos = self.position;

        while self.position < self.input.len()
            && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_')
        {
            self.position += 1;
        }

        let word: String = self.input[starting_pos..self.position].iter().collect();

        match word.as_str() {
            "forward" => Token::Forward,
            "turn"    => Token::Turn,
            "pen"     => Token::Pen,
            "set"     => Token::Set,
            "dotimes" => Token::Dotimes,
            _         => Token::Ident(word),
        }
    }
}

