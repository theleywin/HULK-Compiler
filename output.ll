; ModuleID = 'hulk'
target datalayout = ""
target triple = "x86_64-pc-linux-gnu"
@.str.f = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.str.d = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.s = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
declare i32 @printf(i8* nocapture readonly, ...) nounwind

define i32 @main() {
entry:
  %0 = fadd double 0.0, 5.0
  %1 = fadd double 0.0, 5.0
  %2 = fadd double %0, %1
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.f, i64 0, i64 0), double %2)
  ret i32 0
}