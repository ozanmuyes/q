mod expr;

use expr::Program;

fn print_usage() {
    print!("
q, a calculator - v0.1.0

Usage:
  q \"EXPR\"

Parameters:
  EXPR: Basic mathematical expression string
        that is ASCII-encoded (NOT UTF-8).

Examples:
  q 42
  q \" 42  \"
  q 0xBABE
  q 0o8
  q \"((42 + 0xBABE) - 0o8) * 0b101 / 1\"
  q \"((42 p 0xBABE) m 0o8) x 0b101 d 1\"
  q \"+41 1-0o8*0b101/1\"
")
}

fn main() {
    if let Some(source) = std::env::args().nth(1) {
        let program = Program::new(source);
        // println!("{}", expr.execute()); // FIXME TR bunun yerine Executor gibi bir şey yap, o bilsin AST'nin nasıl çalıştırılacağını
    } else {
        print_usage();
    }
  }
