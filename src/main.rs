use crate::semantic_analyzer::semantic_analyzer::SemanticAnalyzer;
use lalrpop_util::lalrpop_mod;
pub mod ast_nodes;
pub mod semantic_analyzer;
pub mod codegen;
mod tokens;
pub mod types_tree;
pub mod visitor;
use crate::visitor::printer_visitor::PrinterVisitor;
use crate::codegen::CodeGenerator;

lalrpop_mod!(pub parser);

const RUNTIME_C: &str = "runtime.c";

fn main() {
    let input = "5+5";

    let mut expr = parser::ProgramParser::new().parse(input).unwrap();

    let mut printer = PrinterVisitor;
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

            // Compile with runtime.c and math library
            let compile_status = std::process::Command::new("clang")
                .args(&["output.ll", RUNTIME_C, "-lm", "-o", "output"])
                .status()
                .expect("Failed to compile with clang");

            if compile_status.success() {
                // Run the compiled program
                let run_output = std::process::Command::new("./output")
                    .output()
                    .expect("Failed to run program");

                if run_output.status.success() {
                    println!(
                        "\x1b[34mProgram output:\n{}\x1b[0m",
                        String::from_utf8_lossy(&run_output.stdout)
                    );
                } else {
                    eprintln!(
                        "\x1b[31mProgram execution failed:\n{}\x1b[0m",
                        String::from_utf8_lossy(&run_output.stderr)
                    );
                }
            } else {
                // Capture clang error output
                let clang_output = std::process::Command::new("clang")
                    .args(&["output.ll", RUNTIME_C, "-lm", "-o", "output"])
                    .output()
                    .expect("Failed to run clang");

                eprintln!(
                    "\x1b[31mCompilation failed:\n{}\x1b[0m",
                    String::from_utf8_lossy(&clang_output.stderr)
                );
            }
        }
        Err(errors) => {
            println!("\x1b[31mErrors:");
            for err in errors.iter() {
                println!("{}", err.message());
            }
            println!("\x1b[0m");
        }
    }

    println!("");
    // Print AST in blue
    println!("\x1b[34mAST:\n{}\x1b[0m", printer.print_program(&mut expr));
}
