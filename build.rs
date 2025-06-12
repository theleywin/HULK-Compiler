extern crate lalrpop;

fn main() {
    //Generate parser with LALRPOP
    lalrpop::process_root().unwrap();
}
