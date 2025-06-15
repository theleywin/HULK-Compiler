# Makefile — cross-platform LLVM+Clang driver for Hulk compiler

# Detect Windows vs Unix:
ifeq ($(OS),Windows_NT)
    EXEEXT := .exe
    MKDIR_P := mkdir
    RM := rmdir /S /Q
    HULK_RUN := hulk\output$(EXEEXT)
else
    EXEEXT :=
    MKDIR_P := mkdir -p
    RM := rm -rf
    HULK_RUN := ./hulk/output
endif

.PHONY: all compile execute check-deps clean

all: execute

# Build IR + native executable
compile:
	@echo "→ Cleaning previous build..."
	@$(RM) hulk || true
	@$(MKDIR_P) hulk
	@echo "→ Generating LLVM IR and compiling executable via cargo run..."
	@cargo run --release -- script.hulk
	@echo "✓ Build complete. Artifacts in hulk/"

# Run compiled program and display output
execute: compile
	@echo "→ Running hulk program..."
	@echo "==============================================="
	@echo "Program output:"
	@echo "==============================================="
	@$(HULK_RUN) || true
	@echo "==============================================="
	@echo "✓ Program execution complete"

# Cleanup
clean:
	@echo "→ Cleaning up all generated files..."
	@$(RM) hulk output.ll target