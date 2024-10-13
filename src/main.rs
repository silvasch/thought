fn main() {
    if let Err(e) = thought::run() {
        eprintln!("{}", e);
    }
}
