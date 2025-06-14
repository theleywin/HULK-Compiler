use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
use lalrpop_util::lalrpop_mod;
pub mod ast_nodes;
mod parser_w_errors;
pub mod semantic_analyzer;
pub mod codegen;
mod tokens;
pub mod types_tree;
pub mod visitor;

use codegen::CodeGenerator;

lalrpop_mod!(pub parser);

fn main() {
    let input = "
        function Sum(x: Number) : Number => x + 69 ;
        print(Sum(1));
        print( 5 < 4 );
        print(\"La maldita hija del yalorde\");
        print(\"La maldita hija del yalorde\");
        function Fac(x: Number) : Number {
            Sum(x) + 30
        };
        print(Fac(1));
    ";

    let mut expr = parser::ProgramParser::new().parse(input).unwrap();

    let mut semantic_analyzer = SemanticAnalyzer::new();

    println!("");
    let result = semantic_analyzer.analyze(&mut expr);
    match result {
        Ok(_) => {
            println!("Semantic Analyzer OK");

            // Generate code after semantic analysis
            println!("\nGenerating LLVM IR...");
            let mut codegen = CodeGenerator::new();
            let llvm_ir = codegen.generate(&mut expr);

            // Print in green
            println!("\x1b[32mGenerated LLVM IR:\n{}\x1b[0m", llvm_ir);

            // Write to file
            std::fs::write("output.ll", &llvm_ir).expect("Failed to write LLVM IR");
            println!("LLVM IR written to output.ll");

            // Compile and run
            println!("\nCompiling and running...");
            let status = std::process::Command::new("clang")
                .args(&["output.ll", "-o", "output"])
                .status()
                .expect("Failed to compile with clang");

            if status.success() {
                let output = std::process::Command::new("./output")
                    .output()
                    .expect("Failed to run program");
                println!(
                    "\x1b[34mProgram output:\n{}\x1b[0m",
                    String::from_utf8_lossy(&output.stdout)
                );
            } else {
                eprintln!("\x1b[31mCompilation failed\x1b[0m");
            }
        }
        Err(errors) => {
            println!("\x1b[31mSyntax Errors:");
            for err in errors.iter() {
                println!("{}", err);
            }
            println!("\x1b[0m");
        }
    }
}