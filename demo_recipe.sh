#!/bin/bash

echo "═══════════════════════════════════════════════════════════════════"
echo "  UCL: Recipes as Code - Same Logic, Different Substrates"
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "This demonstrates the power of Universal Causal Language:"
echo "The SAME recipe executes on different substrates:"
echo "  • Human Brain VM (simulated cognitive operations)"
echo "  • Robot VM (simulated physical operations)"
echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "Press Enter to see the recipe run on a Human Brain..."
read

cargo run --quiet -- brain examples/recipe_tea.json --verbose

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "Press Enter to see the SAME recipe run on a Robot..."
read

cargo run --quiet -- robot examples/recipe_tea.json --verbose

echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo ""
echo "🎯 Key Insight: Same causal logic, different execution environment!"
echo ""
echo "This enables:"
echo "  • Mock AI + Real Robot arms"
echo "  • Real AI (LLM) + Mock virtual robot"
echo "  • Real AI + Real Robot"
echo ""
echo "The recipe is substrate-independent. Universal. Portable. Composable."
echo "═══════════════════════════════════════════════════════════════════"

