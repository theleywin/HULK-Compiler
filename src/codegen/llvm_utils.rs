use super::context::CodeGenContext;

pub fn declare_printf(output: &mut Vec<String>) {
    output.push("@.str = private unnamed_addr constant [4 x i8] c\"%f\\0A\\00\", align 1".into());
    output.push("declare i32 @printf(i8* nocapture readonly, ...) nounwind".into());
}

pub fn generate_printf(context: &mut CodeGenContext, value: &str) {
    let format_ptr = "i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i64 0, i64 0)";
    context.add_line(format!(
        "call i32 (i8*, ...) @printf({}, double {})",
        format_ptr, value
    ));
}

pub fn generate_header(output: &mut Vec<String>) {
    output.push("; ModuleID = 'hulk'".into());
    output.push("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"".into());
    output.push("target triple = \"x86_64-pc-linux-gnu\"".into());
}

/// Write the main function (no leading spaces in empty or label lines).
pub fn generate_main_wrapper(output: &mut Vec<String>, body: &[String]) {
    output.push("define i32 @main() {".into());
    output.push("entry:".into());
    for line in body {
        // if these lines already have no leading spaces, just push them.
        // if you want to indent instructions, prefix with two spaces here:
        output.push(format!("  {}", line));
    }
    output.push("  ret i32 0".into());
    output.push("}".into());
}

