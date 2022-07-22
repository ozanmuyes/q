#[derive(Debug, PartialEq)]
enum UnaryOperation {
    Negate,
    // MAYBE bitwise ops
    // TODO ...
}

#[derive(Debug, PartialEq)]
enum BinaryOperation {
    Addition,
    Subtraction,
    Multiplication,
    // TODO ...
}

impl From<char> for BinaryOperation {
    fn from(c: char) -> Self {
        match c {
            '+' | 'p' => Self::Addition,
            '-' | 'm' => Self::Subtraction,
            '*' | 'c' => Self::Multiplication,

            _ => todo!()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenVal {
    _CHAR(char), // FIXME This is as a fallback for the unimplemented parts, get rid off of it!

    // LiteralString(String),
    LiteralString(Vec<u8>),
    LiteralDecimal(i32),
    LiteralFloat(f32),
    // MAYBE LiteralDouble(???),
    LiteralHexadecimal(i32),
    LiteralOctal(i32),
    LiteralBinary(i32),
    LiteralBoolean(bool),
    //

    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),

    //
}

#[derive(Debug)]
pub struct Token {
    pub pos: usize,
    pub len: usize,
    pub val: TokenVal,
}

// type AsciiSlice = str;
pub struct AsciiSlice {
    inner: Vec<u8>,
    len: usize,
    next_head: usize,
}

impl AsciiSlice {
    fn len(&self) -> usize {
        self.len
    }
}

impl From<&'static str> for AsciiSlice {
    fn from(string_slice: &'static str) -> Self {
        // MAYBE check on size, because of [DPLCTMM]. say not allow source to have more than 1024 characters

        Self {
            inner: string_slice.to_owned().as_bytes().to_vec(),
            len: string_slice.len(),
            next_head: 0
        }
    }
}

impl From<&String> for AsciiSlice {
    fn from(borrowed_strint: &String) -> Self {
        let inner = borrowed_strint.clone().into_bytes();
        let len = inner.len();

        // MAYBE check on size, because of [DPLCTMM]. say not allow source to have more than 1024 characters

        Self {
            inner,
            len,
            next_head: 0
        }
    }
}

impl Iterator for AsciiSlice {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_head == self.len {
            self.next_head = 0;
            None
        } else {
            let result = self.inner[self.next_head] as char;
            self.next_head += 1;
            Some(result)
        }
    }
}

#[inline]
fn process_buffer(tokens: &mut Vec<Token>, tmp_buf: &Box<[u8]>, tmp_buf_head: usize, tmp_start_pos: Option<usize>) -> usize {
    // let start_pos_sign_omitted: usize = if tmp_buf[0] == '-' as u8 { 1 } else { 0 };
    // let val = if tmp_buf[start_pos_sign_omitted].is_ascii_digit() {
    //     // maybe number
    //     let c: char = tmp_buf[start_pos_sign_omitted + 1] as char;
    //     // match tmp_buf[start_pos_sign_omitted + 1] as char {
    //     match c {
    //         'x' => todo!("hex"),
    //         'o' => todo!("octal"),
    //         'b' => todo!("binary"),
    //         '0'..='9' => TokenVal::LiteralDecimal(String::from_utf8(tmp_buf[0..tmp_buf_head].to_vec()).unwrap().parse::<i32>().expect("ERR_2")),
    //         _ => panic!("Unrecognized base identifier: '{c}'."),
    //     }
    // } else {
    //     // TODO
    //     TokenVal::LiteralString(tmp_buf[0..tmp_buf_head].to_vec())
    // };

    let val = if tmp_buf[0].is_ascii_digit() || tmp_buf[0] == '-' as u8 {
        let buf_str = String::from_utf8(tmp_buf[0..tmp_buf_head].to_vec()).unwrap();
        let _second_char = if tmp_buf[0] == '-' as u8 && tmp_buf_head > 2 {
            tmp_buf[2] as char
        } else {
            tmp_buf[1] as char
        };

        match _second_char {
            '.' => todo!("float"),
            'x' => TokenVal::LiteralHexadecimal(i32::from_str_radix(&buf_str.trim_start_matches("0x"), 16).unwrap()),
            'o' => TokenVal::LiteralOctal(i32::from_str_radix(&buf_str.trim_start_matches("0o"), 8).unwrap()),
            'b' => TokenVal::LiteralBinary(i32::from_str_radix(&buf_str.trim_start_matches("0b"), 2).unwrap()),
            '0'..='9' => TokenVal::LiteralDecimal(buf_str.parse::<i32>().expect("ERR_2")),
            _ => panic!("Unrecognized base identifier: '{_second_char}'."),
        }
    } else {
        // TODO
        TokenVal::LiteralString(tmp_buf[0..tmp_buf_head].to_vec())
    };

    tokens.push(Token {
        pos: tmp_start_pos.expect("ERR_1"),
        len: tmp_buf_head,
        val,
    });

    0
}

pub fn lex(input: AsciiSlice) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let input_len = input.len();

    let mut tmp_start_pos: Option<usize> = None;
    let mut tmp_buf_head: usize = 0;
    // FIXME [DPLCTMM]: Duplicate memory
    //       The use case of `.into_boxed_slice()` is that we don't want it to
    //       grow / shrink. Hence this is not a Vec, an "array" that it's
    //       length only known at runtime.
    //       But it still allocates as the same space on memory as the `source`
    //       does, which I think is to be avoided. The only reason that it is
    //       there me being lazy and unexperienced.
    let mut tmp_buf: Box<[u8]> = vec![0; input.len()].into_boxed_slice();

    for (i, c) in input.enumerate() {
        // do something with character `c` and index `i`
        match c {
            #[rustfmt::skip]
            '0' ..='9' | 'A' ..='F' => {
                tmp_buf[tmp_buf_head] = c as u8;
                tmp_buf_head += 1;
                if tmp_start_pos == None {
                    tmp_start_pos = Some(i);
                }
            },

            #[rustfmt::skip]
            ' ' | '(' | ')' => {
                if tmp_buf_head > 0 {
                    // Push the actual token
                    tmp_buf_head = process_buffer(&mut tokens, &tmp_buf, tmp_buf_head, tmp_start_pos);
                }

                // MAYBE Push the actual token
                // // lexemes.push(Lexeme { pos: i, len: 1, val: c.into() });
                // tokens.push(Token { pos: i, len: 1, val: TokenVal::_CHAR(c) });
                // TODO find out if we have to push the parens or not?!
            }

            #[rustfmt::skip]
            'b' | 'o' | 'x' => {
                tmp_buf[tmp_buf_head] = c as u8;
                tmp_buf_head += 1;
                if tmp_start_pos == None {
                    tmp_start_pos = Some(i);
                }
            }

            //

            '-' => {
                if let Some(pos) = tmp_start_pos {
                    // Handle buffer (if has any value)
                    if tmp_buf_head > 0 {
                        tmp_buf_head = process_buffer(&mut tokens, &tmp_buf, tmp_buf_head, Some(pos));
                    }

                    // Push the actual token
                    tokens.push(Token { pos: i, len: 1, val: TokenVal::BinaryOperation(c.into()) });

                    tmp_start_pos = None;
                } else {
                    // negate
                    tmp_buf[tmp_buf_head] = c as u8;
                    tmp_buf_head += 1;
                    tmp_start_pos = Some(i);
                }
            },

            #[rustfmt::skip]
            '+' /* | '-' */ | '*' | '/' | '%' | '&'
            | 'p' | 'm' | 'c' | 'd' => {
                // Handle buffer (if has any value)
                if tmp_buf_head > 0 {
                    tmp_buf_head = process_buffer(&mut tokens, &tmp_buf, tmp_buf_head, tmp_start_pos);
                }

                // Push the actual token
                tokens.push(Token {
                    pos: i,
                    len: 1,
                    val: TokenVal::BinaryOperation(c.into())
                });

                tmp_start_pos = None;
            },

            _ => println!("unrecognized character: '{}'", &c),
        }

        if i + 1 == input_len && tmp_buf_head > 0 {
            process_buffer(&mut tokens, &tmp_buf, tmp_buf_head, tmp_start_pos);

            // FIXME zero tmp_buf

            break;
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        // Arrange
        let expr: &'static str = "";

        // Act
        let result = lex(expr.into());

        // Assert
        assert!(result.len() == 0)
    }

    // "some unrecognizable string"

    #[test]
    fn test_2() {
        // Arrange
        let expr: &'static str = "1";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 1);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 1);
        assert!(result[0].val == TokenVal::LiteralDecimal(1));
    }

    #[test]
    fn test_3() {
        // Arrange
        let expr: &'static str = "12";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 1);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 2);
        assert!(result[0].val == TokenVal::LiteralDecimal(12));
    }

    #[test]
    fn test_4() {
        // Arrange
        let expr: &'static str = "1234567890";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 1);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 10);
        assert!(result[0].val == TokenVal::LiteralDecimal(1234567890));
    }

    #[test]
    fn test_5() {
        // Arrange
        let expr: &'static str = " 1";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 1);

        assert!(result[0].pos == 1);
        assert!(result[0].len == 1);
        assert!(result[0].val == TokenVal::LiteralDecimal(1));
    }

    #[test]
    fn test_6() {
        // Arrange
        let expr: &'static str = "  1   ";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 1);

        assert!(result[0].pos == 2);
        assert!(result[0].len == 1);
        assert!(result[0].val == TokenVal::LiteralDecimal(1));
    }

    #[test]
    fn test_7() {
        // Arrange
        let expr: &'static str = "1+2";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 3);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 1);
        assert!(result[0].val == TokenVal::LiteralDecimal(1));

        assert!(result[1].pos == 1);
        assert!(result[1].len == 1);
        assert!(result[1].val == TokenVal::BinaryOperation(BinaryOperation::Addition));

        assert!(result[2].pos == 2);
        assert!(result[2].len == 1);
        assert!(result[2].val == TokenVal::LiteralDecimal(2));
    }

    #[test]
    fn test_8() {
        // Arrange
        let expr: &'static str = "1-2";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 3);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 1);
        assert!(result[0].val == TokenVal::LiteralDecimal(1));

        assert!(result[1].pos == 1);
        assert!(result[1].len == 1);
        assert!(result[1].val == TokenVal::BinaryOperation(BinaryOperation::Subtraction));

        assert!(result[2].pos == 2);
        assert!(result[2].len == 1);
        assert!(result[2].val == TokenVal::LiteralDecimal(2));
    }

    #[test]
    fn test_9() {
        // Arrange
        let expr: &'static str = "-1";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 1);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 2);
        assert!(result[0].val == TokenVal::LiteralDecimal(-1));
    }

    #[test]
    fn test_10() {
        // Arrange
        let expr: &'static str = "-1+2";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 3);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 2);
        assert!(result[0].val == TokenVal::LiteralDecimal(-1));

        assert!(result[1].pos == 2);
        assert!(result[1].len == 1);
        assert!(result[1].val == TokenVal::BinaryOperation(BinaryOperation::Addition));

        assert!(result[2].pos == 3);
        assert!(result[2].len == 1);
        assert!(result[2].val == TokenVal::LiteralDecimal(2));
    }

    #[test]
    fn test_11() {
        // Arrange
        let expr: &'static str = "-1-2";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 3);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 2);
        assert!(result[0].val == TokenVal::LiteralDecimal(-1));

        assert!(result[1].pos == 2);
        assert!(result[1].len == 1);
        assert!(result[1].val == TokenVal::BinaryOperation(BinaryOperation::Subtraction));

        assert!(result[2].pos == 3);
        assert!(result[2].len == 1);
        assert!(result[2].val == TokenVal::LiteralDecimal(2));
    }

    #[test]
    fn test_12() {
        // Arrange
        let expr: &'static str = "-1--2";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 3);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 2);
        assert!(result[0].val == TokenVal::LiteralDecimal(-1));

        assert!(result[1].pos == 2);
        assert!(result[1].len == 1);
        assert!(result[1].val == TokenVal::BinaryOperation(BinaryOperation::Subtraction));

        assert!(result[2].pos == 3);
        assert!(result[2].len == 2);
        assert!(result[2].val == TokenVal::LiteralDecimal(-2));
    }

    #[test]
    fn test_13() {
        // Arrange
        let expr: &'static str = "-1- -2";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 3);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 2);
        assert!(result[0].val == TokenVal::LiteralDecimal(-1));

        assert!(result[1].pos == 2);
        assert!(result[1].len == 1);
        assert!(result[1].val == TokenVal::BinaryOperation(BinaryOperation::Subtraction));

        assert!(result[2].pos == 4);
        assert!(result[2].len == 2);
        assert!(result[2].val == TokenVal::LiteralDecimal(-2));
    }

    #[test]
    fn test_14() {
        // Arrange
        let expr: &'static str = "-1 - -2";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 3);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 2);
        assert!(result[0].val == TokenVal::LiteralDecimal(-1));

        assert!(result[1].pos == 3);
        assert!(result[1].len == 1);
        assert!(result[1].val == TokenVal::BinaryOperation(BinaryOperation::Subtraction));

        assert!(result[2].pos == 5);
        assert!(result[2].len == 2);
        assert!(result[2].val == TokenVal::LiteralDecimal(-2));
    }

    // TODO
    // "123"
    // " 123"
    // " 123  "
    // "-123"
    // "-123"

    //

    #[test]
    fn test_h1() {
        // Arrange
        let expr: &'static str = "0xBABE";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 1);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 6);
        assert!(result[0].val == TokenVal::LiteralHexadecimal(0xBABE));
    }

    //

    #[test]
    fn test_o1() {
        // Arrange
        let expr: &'static str = "0o123";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 1);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 5);
        assert!(result[0].val == TokenVal::LiteralOctal(0o123));
    }

    //
    #[test]
    fn test_b1() {
        // Arrange
        let expr: &'static str = "0b101";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 1);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 5);
        assert!(result[0].val == TokenVal::LiteralBinary(0b101));
    }

    //

    #[test]
    fn test_freestyle1() {
        // Arrange
        let expr: &'static str = "0xBABE + 0b101 -42";

        // Act
        let result = lex(expr.into());

        // Assert
        //
        assert!(result.len() == 5);

        assert!(result[0].pos == 0);
        assert!(result[0].len == 6);
        assert!(result[0].val == TokenVal::LiteralHexadecimal(0xBABE));

        assert!(result[1].pos == 7);
        assert!(result[1].len == 1);
        assert!(result[1].val == TokenVal::BinaryOperation(BinaryOperation::Addition));

        assert!(result[2].pos == 9);
        assert!(result[2].len == 5);
        assert!(result[2].val == TokenVal::LiteralBinary(0b101));

        assert!(result[3].pos == 15);
        assert!(result[3].len == 1);
        assert!(result[3].val == TokenVal::BinaryOperation(BinaryOperation::Subtraction));

        assert!(result[4].pos == 16);
        assert!(result[4].len == 2);
        assert!(result[4].val == TokenVal::LiteralDecimal(42));
    }
}
