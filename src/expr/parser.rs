use crate::expr::lexer::{Token, TokenVal};

#[derive(Debug, PartialEq)]
pub enum ASTNodeType {
    LITERAL,
    // TODO
}

#[derive(Debug)]
pub struct AST {
    _type: ASTNodeType,
    // TODO
    nodes: Option<Vec<AST>>,
}

pub fn parse(tokens: Vec<Token>) -> Option<AST> {
    // FIXME
    for token in tokens.into_iter().filter(|l| l.val != TokenVal::_CHAR(' ')) {
        return Some(AST { _type: ASTNodeType::LITERAL, nodes: None });
        // TODO
    }

    None
    // Some(AST { _type: ASTNodeType::LITERAL, nodes: None })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        // Arrange
        let tokens = vec![];

        // Act
        let result = parse(tokens);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn test_2() {
        // Arrange
        let tokens = vec![Token { pos: 0, len: 1, val: TokenVal::LiteralDecimal(1) }];

        // Act
        let result = parse(tokens).unwrap();

        // Assert
        assert!(result._type == ASTNodeType::LITERAL);
        assert!(result.nodes.is_none());
        // TODO
    }

    // TODO
}
