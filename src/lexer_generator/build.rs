fn main() {
    lalrpop::process_root().unwrap();
}
// This will generate the parser code from the grammar files.
// The `lalrpop` crate is used to generate parsers from LALR(1) grammars.
// The `process_root` function processes the root grammar file and generates the necessary code.