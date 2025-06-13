use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
use lalrpop_util::lalrpop_mod;
pub mod ast_nodes;
pub mod semantic_analyzer;
pub mod codegen;
mod tokens;
pub mod types_tree;
pub mod visitor;

lalrpop_mod!(pub parser);

fn main() {
    let input = "type Person (name: String , edad: Number) {
        name = name;
        edad = edad;

        getName(): String => self.name;
        getEdad(): String => self.edad;

    };

    print(new Person(\"Diego\", 22));

    function loca(a: Number): String {
        let b = 5 in (b @ \"Hello world\");
    } ;

    if ( 5 > 4 ) {
        5
    } else {
        \"canchanfleta una pera\"
    };

    let a = 20 in {
        let a = 42 in print(a);
        print(a);
    } ;";

    let mut expr = parser::ProgramParser::new().parse(input).unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    println!("");
    let result = semantic_analyzer.analyze(&mut expr);
    match result {
        Ok(_) => {
            println!("Semantic Analyzer OK");
        }
        Err(errors) => {
            println!("\x1b[31mErrors:");
            for err in errors.iter() {
                println!("{}", err.message());
            }
            println!("\x1b[0m");
        }
    }
}
