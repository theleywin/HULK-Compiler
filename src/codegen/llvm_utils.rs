use super::context::CodeGenContext;
use std::env;

/// Emit the global string constants and the printf declaration.
pub fn declare_printf(output: &mut Vec<String>) {
    output.push(r#"@.str.f = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1"#.into());
    output.push(r#"@.str.d = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1"#.into());
    output.push(r#"@.str.s = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1"#.into());
    output.push("declare i32 @printf(i8* nocapture readonly, ...) nounwind".into());
}

/// Emit a call to printf with the given format and value.
pub fn generate_printf(context: &mut CodeGenContext, value: &str, fmt: &str) {
    let (global_name, arg_type) = match fmt {
        "%f" => ("@.str.f", "double"),
        "%d" => ("@.str.d", "i32"),
        "%s" => ("@.str.s", "i8*"),
        _ => panic!("Unsupported format string: {}", fmt),
    };

    context.add_line(format!(
        "call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* {}, i64 0, i64 0), {} {})",
        global_name,
        arg_type,
        value
    ));
}

/// Emit the module header—ModuleID, data layout, and target triple—dynamically
/// obtained from environment variables set by build.rs.
pub fn generate_header(output: &mut Vec<String>) {
    let datalayout = env::var("LLVM_DATA_LAYOUT")
        .expect("LLVM_DATA_LAYOUT not set; did you run via Cargo with the build script?");
    let triple = env::var("LLVM_HOST_TRIPLE")
        .expect("LLVM_HOST_TRIPLE not set; did you run via Cargo with the build script?");

    output.push(format!("; ModuleID = 'hulk'"));
    output.push(format!(r#"target datalayout = "{}""#, datalayout));
    output.push(format!(r#"target triple = "{}""#, triple));
}

/// Emit the `main` wrapper around the generated body.
pub fn generate_main_wrapper(output: &mut Vec<String>, body: &[String]) {
    output.push("define i32 @main() {".into());
    output.push("entry:".into());
    for line in body {
        output.push(format!("  {}", line));
    }
    output.push("  ret i32 0".into());
    output.push("}".into());
}

/// Emit declarations for runtime helper functions (fmod, pow, concat).
pub fn generate_runtime_declarations(output: &mut Vec<String>) {
    output.push("".into());
    output.push("; Runtime function declarations".into());
    output.push("declare double @fmod(double, double)".into());
    output.push("declare double @pow(double, double)".into());
    output.push("declare i8* @concat(i8*, i8*)".into());
}
