#!/bin/bash
set -e

echo "🚀 Deploying program buffer for upgrade..."

# Configuration
PROGRAM_PATH="./target/deploy/program_upgrade_system.so"
KEYPAIR_PATH="${KEYPAIR_PATH:-~/.config/solana/id.json}"

# Check if program exists
if [ ! -f "$PROGRAM_PATH" ]; then
    echo "❌ Program not found at $PROGRAM_PATH"
    echo "Run 'anchor build' first"
    exit 1
fi

echo "📦 Creating buffer account..."
BUFFER_OUTPUT=$(solana program write-buffer \
    --buffer-authority "$KEYPAIR_PATH" \
    "$PROGRAM_PATH" \
    --keypair "$KEYPAIR_PATH")

# Extract buffer address
BUFFER_ADDRESS=$(echo "$BUFFER_OUTPUT" | grep -oE '[A-Za-z0-9]{32,44}' | head -1)

echo "✅ Buffer created: $BUFFER_ADDRESS"
echo ""
echo "Next steps:"
echo "1. Verify the buffer: solana program show $BUFFER_ADDRESS"
echo "2. Propose upgrade with this buffer address"
echo ""
echo "Buffer Address: $BUFFER_ADDRESS"
