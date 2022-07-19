fn main() {
    if let Some(expr) = std::env::args().nth(1) {
        todo!("calculate '{}'", expr)
    } else {
        todo!("print info and usage")
    }
}
