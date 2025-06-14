#!/bin/bash

# ente rust file name (without .rs extension)
read -p "Enter the Rust file name (without .rs): " filename

# check if the Rust source file exists
if [ ! -f "$filename.rs" ]; then
    echo "Error: $filename.rs not found!"
    exit 1
fi

# compile the Rust source file and create the binary
rustc "$filename.rs"

# check if the first compilation succeeded
if [ $? -ne 0 ]; then
    echo "Error: Compilation failed!"
    exit 1
fi

# if the binary already exists, remove it
if [ -f "$filename" ]; then
    rm "$filename"
else
    echo "Warning: Binary file $filename not found to remove."
fi

# compile the rust source file again
rustc "$filename.rs"

# check if the second compilation succeeded
if [ $? -ne 0 ]; then
    echo "Error: Compilation failed again!"
    exit 1
fi

# give execution permission to the compiled binary
chmod +x "$filename"

# run the compiled binary
if [ -f "$filename" ]; then
    ./"$filename"
else
    echo "Error: $filename binary not found!"
    exit 1
fi

rm $filename
