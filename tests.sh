#!/bin/bash

# Directory containing the test files
TESTS_DIR="./tests"

# Iterate over all .rs files in the tests directory
for file in "$TESTS_DIR"/*.rs; do
    # Extract the file name without the extension
    test_name=$(basename "$file" .rs)
    
    # Run the cargo test command with the test name
    echo "Running test: $test_name"
    cargo test --test "$test_name"
done