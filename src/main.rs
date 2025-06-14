use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
use lalrpop_util::lalrpop_mod;
pub mod ast_nodes;
mod parser_w_errors;
pub mod semantic_analyzer;
pub mod codegen;
mod tokens;
pub mod types_tree;
pub mod visitor;

lalrpop_mod!(pub parser);

use crate::parser_w_errors::Parser;
use codegen::CodeGenerator;

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
       
        print(\"another test aaahhagahahhh sexo\");
        let x = 59 in {
            while(x%69 != 0) {
                x := x + 1;
                print(x)
            };
        };    
";

     let parser = Parser::new();
    match parser.parse(input) {
        Ok(mut expr) => {
            println!("\x1b[32mSyntactic Analyzer OK\x1b[0m");
            println!("\x1b[0m");
            let mut semantic_analyzer = SemanticAnalyzer::new();
            let result = semantic_analyzer.analyze(&mut expr);
            match result {
                Ok(_) => {
                    println!("\x1b[32mSemantic Analyzer OK\x1b[0m");
                }
                Err(errors) => {
                    println!("\x1b[31mSemantic Errors:");
                    for err in errors.iter() {
                        println!("{}", err.message());
                    }
                    println!("\x1b[0m");
                }
            }
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