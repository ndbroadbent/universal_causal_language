#!/bin/bash
# Demo script for UCL CLI

set -e

echo "========================================"
echo "  Universal Causal Language (UCL) Demo"
echo "========================================"
echo ""

UCL="./target/release/ucl"

if [ ! -f "$UCL" ]; then
    echo "Building UCL..."
    cargo build --release
    echo ""
fi

echo "1. Validating all examples..."
echo "------------------------------"
for file in examples/*.json; do
    echo "  ✓ $(basename $file)"
    $UCL validate "$file" > /dev/null
done
echo ""

echo "2. Displaying Natural Language example..."
echo "------------------------------------------"
$UCL display examples/natural_language.json
echo ""

echo "3. Analyzing Biology example..."
echo "--------------------------------"
$UCL analyze examples/biology.json
echo ""

echo "4. Displaying Music example (compact)..."
echo "------------------------------------------"
$UCL display --compact examples/music.json | head -20
echo "..."
echo ""

echo "5. Analyzing Ruby code example..."
echo "----------------------------------"
$UCL analyze examples/ruby_code.json
echo ""

echo "========================================"
echo "  Demo complete!"
echo "========================================"
echo ""
echo "Try these commands:"
echo "  ucl validate examples/legal_contract.json"
echo "  ucl display examples/rust_code.json"
echo "  ucl analyze examples/music.json"
echo ""

