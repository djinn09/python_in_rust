#!/bin/bash

# This script sets up the Python virtual environment and installs dependencies.

echo "Creating Python virtual environment..."
python3 -m venv .venv

echo "Installing dependencies..."
.venv/bin/pip install libcst

echo "Setup complete."
