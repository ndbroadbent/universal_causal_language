#!/bin/bash
# Advanced demo showing Ruby compilation and Brain VM

set -e

echo "=========================================="
echo "  UCL Advanced Features Demo"
echo "  Ruby Compiler + Brain VM"
echo "=========================================="
echo ""

UCL="./target/release/ucl"

if [ ! -f "$UCL" ]; then
    echo "Building UCL..."
    cargo build --release
    echo ""
fi

echo "=== Part 1: Ruby Compilation ==="
echo ""

echo "1. Compiling Hello World to Ruby..."
echo "------------------------------------"
$UCL compile examples/hello_world.json --target ruby
echo ""

echo "2. Running Hello World..."
echo "-------------------------"
$UCL run examples/hello_world.json --target ruby
echo ""

echo "=== Part 2: Brain VM ==="
echo ""

echo "3. Executing natural language on Brain VM..."
echo "---------------------------------------------"
$UCL brain examples/natural_language.json
echo ""

echo "4. Brain VM with verbose output..."
echo "-----------------------------------"
$UCL brain examples/natural_language.json --verbose | head -50
echo ""

echo "5. Testing brain confusion (unknown operations)..."
echo "---------------------------------------------------"
$UCL brain examples/confusion_test.json --verbose
echo ""

echo "=== Part 3: Cross-Target Execution ==="
echo ""

echo "6. Same program, different substrates..."
echo "-----------------------------------------"
echo "Creating a simple program..."
cat > /tmp/test_ucl.json << 'EOF'
{
  "actions": [
    {"actor": "system", "op": "Emit", "target": "message",
     "params": {"content": "UCL works!"}}
  ]
}
EOF

echo ""
echo "Ruby target:"
$UCL run /tmp/test_ucl.json --target ruby
echo ""
echo "Brain target:"
$UCL run /tmp/test_ucl.json --target brain | grep "Output/Speech" -A 2
echo ""

echo "=========================================="
echo "  Demo complete!"
echo "=========================================="
echo ""
echo "Key Insights:"
echo "  • UCL compiles to multiple targets (Ruby, Brain VM)"
echo "  • Language literally executes as programs on the brain"
echo "  • Unknown operations trigger natural confusion responses"
echo "  • Same causal logic, different execution substrates"
echo ""
echo "Learn more:"
echo "  - README.md: Full documentation"
echo "  - BRAIN_VM.md: The revolutionary brain VM concept"
echo "  - examples/: More UCL programs to try"
echo ""

