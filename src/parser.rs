use crate::{
    ast::{BinopKind, Expr, Stmt},
    lexer::Token,
};

pub(crate) type Result<T = ()> = std::result::Result<T, ErrorKind>;

#[derive(Clone, Debug)]
pub(crate) enum ErrorKind {
    UnexpectedToken(Token),
    UnexpectedEndOfInput(usize),
}

pub(crate) struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub(crate) fn parse(&self) -> Result<Vec<Stmt>> {
        let mut exprs = vec![];

        let mut parsed = self.parse_binding(&self.tokens, 0);

        while let Ok((expr, pos)) = parsed {
            exprs.push(expr);

            parsed = self.parse_binding(&self.tokens, pos);
        }

        Ok(exprs)
    }

    fn parse_binding(&self, tokens: &[Token], pos: usize) -> Result<(Stmt, usize)> {
        match (tokens.get(pos), tokens.get(pos + 1), tokens.get(pos + 2)) {
            (Some(Token::Let), Some(Token::Ident(name)), Some(Token::Equal)) => {
                let (expr, pos) = self.parse_expr(tokens, pos + 3)?;

                Ok((Stmt::Binding(name.to_string(), expr), pos))
            }
            _ => Err(ErrorKind::UnexpectedEndOfInput(pos)) 
        }
    }

    fn parse_expr(&self, tokens: &[Token], pos: usize) -> Result<(Expr, usize)> {
        let (left, pos) = self.parse_term(tokens, pos)?;

        match tokens.get(pos) {
            Some(Token::Plus) => {
                let (right, pos) = self.parse_term(tokens, pos + 1)?;

                Ok((
                    Expr::Binop(BinopKind::Plus, Box::new(left), Box::new(right)),
                    pos,
                ))
            }
            Some(Token::Minus) => {
                let (right, pos) = self.parse_term(tokens, pos + 1)?;

                Ok((
                    Expr::Binop(BinopKind::Minus, Box::new(left), Box::new(right)),
                    pos,
                ))
            }
            _ => Ok((left, pos)),
        }
    }

    fn parse_term(&self, tokens: &[Token], pos: usize) -> Result<(Expr, usize)> {
        let (left, pos) = self.parse_literal(tokens, pos)?;

        match tokens.get(pos) {
            Some(Token::Times) => {
                let (right, pos) = self.parse_literal(tokens, pos + 1)?;

                Ok((
                    Expr::Binop(BinopKind::Times, Box::new(left), Box::new(right)),
                    pos,
                ))
            }
            Some(Token::Divide) => {
                let (right, pos) = self.parse_literal(tokens, pos + 1)?;

                Ok((
                    Expr::Binop(BinopKind::Divide, Box::new(left), Box::new(right)),
                    pos,
                ))
            }
            _ => Ok((left, pos)),
        }
    }

    fn parse_literal(&self, tokens: &[Token], pos: usize) -> Result<(Expr, usize)> {
        match tokens.get(pos) {
            Some(Token::LBracket) => {
                self.parse_expr(tokens, pos + 1)
                    .and_then(|(expr, pos)| match tokens.get(pos) {
                        Some(Token::RBracket) => Ok((expr, pos + 1)),
                        Some(token) => Err(ErrorKind::UnexpectedToken(token.clone())),
                        None => Err(ErrorKind::UnexpectedEndOfInput(pos)),
                    })
            }
            Some(Token::Number(num)) => Ok((Expr::Number(*num), pos + 1)),
            Some(token) => Err(ErrorKind::UnexpectedToken(token.clone())),
            None => Err(ErrorKind::UnexpectedEndOfInput(pos)),
        }
    }
}
