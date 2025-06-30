fn main() {
    lalrpop::Configuration::new()
        .set_in_dir("src/regex/parser")
        .process()
        .unwrap();
}
