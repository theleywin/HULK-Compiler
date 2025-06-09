; ModuleID = 'hulk'
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"
@.str = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
declare i32 @printf(i8* nocapture readonly, ...) nounwind

define i32 @main() {
entry:
  %0 = fadd double 5.0, 5.0
  %1 = fsub double 2.0, 1.0
  %2 = fdiv double 3.0, %1
  %3 = fmul double %2, 7.0
  %4 = fsub double %0, %3
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i64 0, i64 0), double %4)
  ret i32 0
}