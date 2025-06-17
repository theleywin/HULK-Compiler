use super::context::CodeGenContext;

/// Emit the global string constants and the printf declaration.
pub fn declare_printf(output: &mut Vec<String>,  context: &mut CodeGenContext) {
    output.push("@PI = constant double 0x400921FB54442D18".into()); // π
    output.push("@E = constant double 0x4005BF0A8B145769".into()); // e
    context.add_global_constant("PI");
    context.add_global_constant("E");
    output.push(r#"@.str.f = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1"#.into());
    output.push(r#"@.str.d = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1"#.into());
    output.push(r#"@.str.s = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1"#.into());
    output.push(r#"@.true_str = private  constant [6 x i8] c"true\0A\00", align 1"#.into());
    output.push(r#"@.false_str = private constant [7 x i8] c"false\0A\00", align 1"#.into());
    output.push(r#"@.newline = private unnamed_addr constant [2 x i8] c"\0A\00", align 1"#.into());
    output.push("declare i32 @printf(ptr, ...)".into());
    output.push("declare i32 @strlen( ptr )".into());
    output.push("declare ptr @strcpy(ptr,ptr)".into());
    output.push("declare ptr @strcat(ptr,ptr)".into());
    output.push("declare i32 @strcmp(ptr ,ptr)".into());
    output.push("declare i8* @malloc(i64)".into());
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
    output.push("; ModuleID = 'hulk'".into());
    output.push("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"".into());
    output.push("target triple = \"x86_64-pc-linux-gnu\"".into());
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

pub fn to_llvm_type(type_node: String) -> String {
    match type_node.as_str() {
        "Number" => "double".to_string(),
        "Boolean" => "i1".to_string(),
        "String" => "ptr".to_string(),
        _ => format!("%{}_type*", type_node), // Default to pointer type for unknown types
    }
}
