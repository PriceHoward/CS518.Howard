#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Forward,
    Turn,
    Pen,
    Set,
    Dotimes,
    LBrace,
    RBrace,
    Number(f64),
    Ident(String),
}