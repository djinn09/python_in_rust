#!/bin/bash

# This script sets up the development environment for the project.

echo "--- Setting up Python virtual environment ---"
python3 -m venv .venv
echo "Virtual environment created at ./.venv"

echo ""
echo "--- Installing Python dependencies ---"
source .venv/bin/activate
pip install libcst
deactivate

echo ""
echo "--- Checking for Rust installation ---"
if ! command -v cargo &> /dev/null
then
    echo "Rust is not installed. Please install it from https://rustup.rs and try again."
    exit 1
fi

echo "Rust is installed. Building the project to fetch and compile Rust dependencies..."
cargo build

echo ""
echo "Setup complete. You can now run the examples."
echo "For the PyO3 examples, make sure to activate the virtual environment first:"
echo "source .venv/bin/activate"
