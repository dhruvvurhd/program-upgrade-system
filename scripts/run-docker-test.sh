#!/bin/bash
#===============================================================================
# Program Upgrade System - Docker-based Test Runner
# Runs tests in an isolated Docker container with all dependencies
#===============================================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🐳 Program Upgrade System - Docker Test Runner"
echo ""

cd "$PROJECT_ROOT"

# Use a comprehensive Solana development image
# solanalabs/solana contains everything needed
docker run --rm \
    -v "${PWD}:/workspace" \
    -w /workspace \
    --name program-upgrade-test \
    solanalabs/solana:v1.18.17 \
    bash -c '
        set -e
        
        echo "📦 Installing Node.js..."
        apt-get update -qq && apt-get install -y -qq curl > /dev/null 2>&1
        curl -fsSL https://deb.nodesource.com/setup_18.x | bash - > /dev/null 2>&1
        apt-get install -y -qq nodejs > /dev/null 2>&1
        
        echo "✅ Node version: $(node --version)"
        echo "✅ Solana version: $(solana --version)"
        
        echo ""
        echo "📦 Installing Anchor CLI..."
        curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y > /dev/null 2>&1
        source ~/.cargo/env
        cargo install --git https://github.com/coral-xyz/anchor --tag v0.32.1 anchor-cli 2>&1 | tail -5
        echo "✅ Anchor version: $(anchor --version)"
        
        echo ""
        echo "📦 Installing Node dependencies..."
        npm install --legacy-peer-deps 2>&1 | tail -3
        
        echo ""
        echo "🔧 Configuring Solana..."
        solana-keygen new --no-bip39-passphrase -o ~/.config/solana/id.json --force
        
        echo ""
        echo "🏗️ Building program..."
        anchor build 2>&1 | tail -10
        
        echo ""
        echo "🚀 Starting validator and running tests..."
        solana-test-validator --reset --quiet &
        VALIDATOR_PID=$!
        sleep 10
        
        solana config set -u localhost
        solana airdrop 100 || true
        
        echo ""
        echo "🧪 Running Anchor tests..."
        anchor test --skip-local-validator
        
        kill $VALIDATOR_PID 2>/dev/null || true
        echo ""
        echo "✅ All tests completed!"
    '
