use super::lexer::*;

pub struct ASTParser<'a> {
    reader: TokenStreamReader<'a>,
}

struct TokenStreamReader<'a> {
    src: &'a Vec<Token>,
}
impl<'a> TokenStreamReader<'a> {

}

impl<'a> ASTParser<'a> {
    pub fn parse() -> Result<ASTNode, ASTParsingError> {
        todo!()
    }
}

pub struct ASTParsingError {

}

pub struct ASTNode {
    
}
