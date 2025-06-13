# Makefile — cross-platform LLVM+Clang driver for Hulk compiler

# Detect Windows vs Unix:
ifeq ($(OS),Windows_NT)
    EXEEXT := .exe
    MKDIR_P := mkdir
    RM := rmdir /S /Q
    HULK_RUN := hulk\hulk$(EXEEXT)
else
    EXEEXT :=
    MKDIR_P := mkdir -p
    RM := rm -rf
    HULK_RUN := ./hulk/hulk
endif

.PHONY: all compile execute check-deps clean

all: execute

#Ensure required tools are installed
check-deps:
	@command -v llvm-config >/dev/null 2>&1 || (echo "Error: llvm-config not found. Please install LLVM/Clang." >&2; exit 1)
	@command -v clang        >/dev/null 2>&1 || (echo "Error: clang not found. Please install LLVM/Clang."  >&2; exit 1)
	@command -v cargo        >/dev/null 2>&1 || (echo "Error: cargo not found. Please install Rust toolchain." >&2; exit 1)

#Build IR + native executable
compile: check-deps hulk/hulk$(EXEEXT)

hulk/hulk$(EXEEXT): script.hulk
	@echo "→ Cleaning previous build..."
	@$(RM) hulk || true
	@$(MKDIR_P) hulk
	@echo "→ Generating LLVM IR via cargo run..."
	cargo run --release -- script.hulk
	@echo "→ Moving IR to hulk/output.ll"
	mv output.ll hulk/output.ll
	@echo "→ Compiling with clang → native executable"
	clang hulk/output.ll runtime.c -o hulk/hulk$(EXEEXT)
	@echo "✓ Build complete. Artifacts in hulk/"

#Run compiled program
execute: compile
	@echo "→ Running hulk..."
	@$(HULK_RUN)

#Cleanup
clean:
	@echo "→ Cleaning up all generated files..."
	@$(RM) hulk output.ll target
