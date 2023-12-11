#[derive(Debug, Clone, PartialEq)]
enum Token {
    Word(String),
    Pipe,
    Semicolon,
    And,
    Or,
}

#[derive(Debug, Clone)]
struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.consume_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        match self.input.chars().nth(self.position).unwrap() {
            ';' => {
                self.position += 1;
                Some(Token::Semicolon)
            }
            '&' => {
                self.position += 1;
                if let Some(c) = self.input.chars().nth(self.position) {
                    if c == '&' {
                        self.position += 1;
                        Some(Token::And)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            '|' => {
                self.position += 1;
                if let Some(c) = self.input.chars().nth(self.position) {
                    if c == '|' {
                        self.position += 1;
                        Some(Token::Or)
                    } else {
                        Some(Token::Pipe)
                    }
                } else {
                    None
                }
            }
            _ => {
                let start = self.position;
                while let Some(c) = self.input.chars().nth(self.position) {
                    if c.is_whitespace() || c == '|' || c == ';' || c == '&' {
                        break;
                    }
                    self.position += 1;
                }
                let word = self.input[start..self.position].to_owned();
                Some(Token::Word(word))
            }
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.input.chars().nth(self.position) {
            if c.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }
}

#[derive(Debug)]
enum AST {
    Command(String),
    Pipeline(Box<AST>, Box<AST>),
    List(Box<AST>, Token, Box<AST>),
}

#[derive(Clone)]
struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: None,
        };
        parser.consume_token();
        parser
    }

    fn consume_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn match_token(&mut self, expected: Token) -> bool {
        if self.current_token == Some(expected.clone()) {
            self.consume_token();
            true
        } else {
            false
        }
    }

    fn parse_word(&mut self) -> Option<String> {
        match self.clone().current_token {
            Some(Token::Word(word)) => {
                self.consume_token();
                Some(word.clone())
            }
            _ => None,
        }
    }

    fn parse_simple_command(&mut self) -> Option<AST> {
        if let Some(word) = self.parse_word() {
            Some(AST::Command(word))
        } else {
            None
        }
    }

    fn parse_pipeline(&mut self) -> Option<AST> {
        if let Some(command1) = self.parse_simple_command() {
            if self.match_token(Token::Pipe) {
                if let Some(command2) = self.parse_simple_command() {
                    Some(AST::Pipeline(Box::new(command1), Box::new(command2)))
                } else {
                    None
                }
            } else {
                Some(command1)
            }
        } else {
            None
        }
    }

    fn parse_list(&mut self) -> Option<AST> {
        if let Some(command1) = self.parse_pipeline() {
            if self.match_token(Token::Semicolon)
                || self.match_token(Token::And)
                || self.match_token(Token::Or)
            {
                if let Some(command2) = self.parse_list() {
                    let operator = match &self.current_token {
                        Some(token) => token.clone(),
                        None => return Some(command1),
                    };
                    self.consume_token();
                    Some(AST::List(Box::new(command1), operator, Box::new(command2)))
                } else {
                    None
                }
            } else {
                Some(command1)
            }
        } else {
            None
        }
    }

    fn parse(&mut self) -> Option<AST> {
        self.parse_list()
    }
}

fn main() {
    let input = "echo hello ; ls | grep rs";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    if let Some(ast) = parser.parse() {
        println!("{:?}", ast);
    } else {
        println!("Parsing failed");
    }
}
