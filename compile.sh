#!/bin/bash

# Script to compile both C and Rust files in all subdirectories and measure compile time

echo "Starting compilation benchmarks..."

# Loop through all directories
for dir in [0-9]*-*/; do
  # Remove trailing slash
  dir=${dir%/}
  
  echo "===== Directory: $dir ====="
  
  # Change to the directory
  cd "$dir"
  
  # Compile C code with time
  echo "C compilation:"
  time gcc code.c -o c
  
  # Compile Rust code with time
  echo "Rust compilation:"
  time rustc code.rs -o rust
  
  # Return to parent directory
  cd ..
  
  echo ""
done

echo "All compilations completed."