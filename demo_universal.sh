#!/bin/bash
# Demo: Same UCL program running on THREE different substrates

set -e

echo "=========================================="
echo "  UNIVERSAL PROGRAM DEMO"
echo "  Same Causal Logic, Three Substrates"
echo "=========================================="
echo ""
echo "Program: Generate two random numbers and multiply them"
echo ""

UCL="./target/release/ucl"

echo "=== 1. RUBY VM (Silicon) ==="
echo "Running 3 times to show randomness:"
echo ""
for i in {1..3}; do
  result=$($UCL run examples/multiply_universal.json --target ruby 2>&1 | grep "Execution Output" -A 1 | tail -1)
  echo "  Run $i: $result"
done
echo ""

echo "Compiled Ruby code:"
echo "-------------------"
$UCL compile examples/multiply_universal.json --target ruby | tail -5
echo ""

echo "=== 2. BRAIN VM (Simulated Wetware) ==="
echo "Running 3 times to show randomness:"
echo ""
for i in {1..3}; do
  result=$($UCL brain examples/multiply_universal.json 2>&1 | grep "Output/Speech" -A 1 | tail -1 | sed 's/^[[:space:]]*//')
  echo "  Run $i: $result"
done
echo ""

echo "Detailed execution with brain state:"
echo "-------------------------------------"
$UCL brain examples/multiply_universal.json --verbose
echo ""

echo "=== 3. PRODUCTION BRAIN (Your Actual Neurons) ==="
echo ""
echo "Ready to run this program on YOUR brain?"
echo "This will be interactive - you'll execute each operation."
echo ""
read -p "Run on production brain? (y/n): " answer
echo ""

if [[ "$answer" =~ ^[Yy]$ ]]; then
  $UCL brain examples/multiply_universal.json --production
else
  echo "Skipped production brain execution."
  echo ""
  echo "You can run it later with:"
  echo "  $UCL brain examples/multiply_universal.json --production"
fi

echo ""
echo "=========================================="
echo "  DEMO COMPLETE"
echo "=========================================="
echo ""
echo "💡 Key Insight:"
echo "  The SAME UCL program executed on:"
echo "    1. Ruby VM (compiled to Ruby, executed on silicon)"
echo "    2. Brain VM (simulated human cognition)"
echo "    3. Production Brain (YOUR actual neurons)"
echo ""
echo "  Same causal logic."
echo "  Different substrates."
echo "  Universal computation. 🚀"
echo ""

