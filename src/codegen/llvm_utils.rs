use super::context::CodeGenContext;

pub fn declare_printf(output: &mut Vec<String>) {
    // Format string for double: "%f\n"
    output.push("@.str.f = private unnamed_addr constant [4 x i8] c\"%f\\0A\\00\", align 1".into());
    // Format string for integer: "%d\n"
    output.push("@.str.d = private unnamed_addr constant [4 x i8] c\"%d\\0A\\00\", align 1".into());
    // Format string for string: "%s\n"
    output.push("@.str.s = private unnamed_addr constant [4 x i8] c\"%s\\0A\\00\", align 1".into());
    output.push("declare i32 @printf(i8* nocapture readonly, ...) nounwind".into());
}

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

pub fn generate_header(output: &mut Vec<String>) {
    output.push("; ModuleID = 'hulk'".into());
    output.push("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"".into());
    output.push("target triple = \"x86_64-pc-linux-gnu\"".into());
}

pub fn generate_main_wrapper(output: &mut Vec<String>, body: &[String]) {
    output.push("define i32 @main() {".into());
    output.push("entry:".into());
    for line in body {
        output.push(format!("  {}", line));
    }
    output.push("  ret i32 0".into());
    output.push("}".into());
}
