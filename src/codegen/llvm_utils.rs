use super::context::CodeGenContext;

pub fn declare_printf(context: &mut CodeGenContext) {
    context
        .add_line("@.str = private unnamed_addr constant [4 x i8] c\"%f\\0A\\00\", align 1".into());
    context.add_line("declare i32 @printf(i8* nocapture readonly, ...) nounwind".into());
}

pub fn generate_printf(context: &mut CodeGenContext, value: &str) {
    let format_ptr = "i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i64 0, i64 0)";
    context.add_line(format!(
        "call i32 (i8*, ...) @printf({}, double {})",
        format_ptr, value
    ));
}

pub fn generate_header(context: &mut CodeGenContext) {
    context.add_line("; ModuleID = 'hulk'".into());
    context.add_line("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"".into());
    context.add_line("target triple = \"x86_64-pc-linux-gnu\"".into());
}

pub fn generate_main_wrapper(context: &mut CodeGenContext, body: &[String]) {
    context.add_line("define i32 @main() {".into());
    context.add_line("entry:".into());
    for line in body {
        context.add_line(line.clone());
    }
    context.add_line("ret i32 0".into());
    context.add_line("}".into());
}
