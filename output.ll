; ModuleID = 'hulk'
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"
@.str.f = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1
@.str.d = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.s = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
declare i32 @printf(i8* nocapture readonly, ...) nounwind

declare double @fmod(double, double)

define i32 @main() {
entry:
  %0 = call double @fmod(double 5.0, double 5.0)
  call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str.f, i64 0, i64 0), double %0)
  ret i32 0
}