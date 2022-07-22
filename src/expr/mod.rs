mod lexer;
mod parser;

use lexer::{lex, AsciiSlice};
use parser::{AST, parse};

pub struct Program {
    source: String,
    ast: AST,
    //
}

impl Program {
    pub fn new(source: String) -> Program {
        // &source.into::<AsciiSlice>()
        let lxms = lex(AsciiSlice::from(&source));
        // println!("{:#?}", lxms);

        let ast = parse(lxms).expect("AST error");
        // println!("{:#?}", ast);

        //

        Program {
            source,
            ast,
        }
    }

    // TODO MAYBE
}
