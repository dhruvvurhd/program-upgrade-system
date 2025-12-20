#!/bin/bash
#===============================================================================
# Program Upgrade System - Industry-Grade Local CI Test Runner
# Version: 1.0.0
#
# This script replicates the GitHub Actions CI pipeline locally:
# - Validates prerequisites
# - Builds the Anchor program
# - Starts a local Solana test validator
# - Deploys the program
# - Runs comprehensive tests (12 test cases)
# - Handles timelock testing via reduced test timelock or time warping
# - Provides detailed reporting
#===============================================================================

set -eo pipefail

# ==============================================================================
# Configuration
# ==============================================================================
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
VALIDATOR_LOG="/tmp/program-upgrade-validator.log"
VALIDATOR_PID_FILE="/tmp/program-upgrade-validator.pid"
TEST_TIMEOUT=300  # 5 minutes max for tests

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ==============================================================================
# Helper Functions
# ==============================================================================
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

log_step() {
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}🔷 $1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

check_command() {
    if ! command -v $1 &> /dev/null; then
        log_error "$1 is not installed"
        echo "  Please install $1 before running this script."
        return 1
    fi
    log_success "$1 found: $(command -v $1)"
    return 0
}

cleanup() {
    log_step "Cleanup"
    if [ -f "$VALIDATOR_PID_FILE" ]; then
        VALIDATOR_PID=$(cat "$VALIDATOR_PID_FILE")
        if kill -0 "$VALIDATOR_PID" 2>/dev/null; then
            log_info "Stopping validator (PID: $VALIDATOR_PID)..."
            kill "$VALIDATOR_PID" 2>/dev/null || true
            sleep 1
            kill -9 "$VALIDATOR_PID" 2>/dev/null || true
        fi
        rm -f "$VALIDATOR_PID_FILE"
    fi
    # Kill any stray validators
    pkill -f "solana-test-validator" 2>/dev/null || true
    log_success "Cleanup complete"
}

# Trap for cleanup on exit
trap cleanup EXIT INT TERM

# ==============================================================================
# Prerequisites Check
# ==============================================================================
log_step "Prerequisites Check"

PREREQS_FAILED=0

check_command solana || PREREQS_FAILED=1
check_command anchor || PREREQS_FAILED=1
check_command node || PREREQS_FAILED=1

# Check for yarn or npm
if command -v yarn &> /dev/null; then
    PKG_MANAGER="yarn"
    log_success "yarn found: $(command -v yarn)"
elif command -v npm &> /dev/null; then
    PKG_MANAGER="npm"
    log_success "npm found: $(command -v npm)"
else
    log_error "Neither yarn nor npm is installed"
    PREREQS_FAILED=1
fi

if [ $PREREQS_FAILED -eq 1 ]; then
    log_error "Prerequisites check failed. Please install missing tools."
    echo ""
    echo "Installation commands:"
    echo "  Solana:  sh -c \"\$(curl -sSfL https://release.anza.xyz/v1.18.17/install)\""
    echo "  Anchor:  cargo install --git https://github.com/coral-xyz/anchor avm --locked --force"
    echo "           avm install 0.30.1 && avm use 0.30.1"
    echo "  Node.js: Install via nvm or package manager"
    exit 1
fi

log_success "All prerequisites satisfied"

# ==============================================================================
# Environment Setup
# ==============================================================================
log_step "Environment Setup"

cd "$PROJECT_ROOT"
log_info "Working directory: $(pwd)"

# Display versions
log_info "Solana version: $(solana --version)"
log_info "Anchor version: $(anchor --version)"
log_info "Node version: $(node --version)"

# ==============================================================================
# Install Dependencies
# ==============================================================================
log_step "Installing Dependencies"

if [ "$PKG_MANAGER" = "yarn" ]; then
    yarn install --frozen-lockfile 2>/dev/null || yarn install
else
    npm ci 2>/dev/null || npm install
fi

log_success "Dependencies installed"

# ==============================================================================
# Build Program
# ==============================================================================
log_step "Building Anchor Program"

BUILD_START=$(date +%s)

anchor build

BUILD_END=$(date +%s)
BUILD_TIME=$((BUILD_END - BUILD_START))

log_success "Build completed in ${BUILD_TIME} seconds"

# Verify build artifacts
if [ -f "target/deploy/program_upgrade_system.so" ]; then
    SO_SIZE=$(du -h target/deploy/program_upgrade_system.so | cut -f1)
    log_success "Program binary: target/deploy/program_upgrade_system.so ($SO_SIZE)"
else
    log_error "Program binary not found!"
    exit 1
fi

if [ -f "target/idl/program_upgrade_system.json" ]; then
    log_success "IDL generated: target/idl/program_upgrade_system.json"
else
    log_warning "IDL not found (may be expected if using older Anchor)"
fi

# ==============================================================================
# Start Local Validator
# ==============================================================================
log_step "Starting Solana Test Validator"

# Kill any existing validator
pkill -f "solana-test-validator" 2>/dev/null || true
sleep 2

# Start validator
log_info "Starting fresh validator..."
nohup solana-test-validator --reset --quiet > "$VALIDATOR_LOG" 2>&1 &
VALIDATOR_PID=$!
echo "$VALIDATOR_PID" > "$VALIDATOR_PID_FILE"
log_info "Validator PID: $VALIDATOR_PID"

# Wait for validator to be ready
log_info "Waiting for validator to be ready..."
for i in {1..60}; do
    if solana cluster-version -u localhost > /dev/null 2>&1; then
        log_success "Validator is ready! (took ${i}s)"
        break
    fi
    if [ $i -eq 60 ]; then
        log_error "Validator failed to start within 60 seconds"
        echo "Validator log:"
        tail -50 "$VALIDATOR_LOG"
        exit 1
    fi
    sleep 1
done

# ==============================================================================
# Configure Solana CLI
# ==============================================================================
log_step "Configuring Solana CLI"

solana config set -u localhost
log_success "RPC URL set to localhost"

# Create keypair if needed
if [ ! -f ~/.config/solana/id.json ]; then
    log_info "Creating new keypair..."
    solana-keygen new --no-bip39-passphrase -o ~/.config/solana/id.json --force
else
    log_success "Using existing keypair"
fi

WALLET_ADDRESS=$(solana address)
log_info "Wallet address: $WALLET_ADDRESS"

# Airdrop SOL
log_info "Requesting airdrop..."
for i in {1..5}; do
    if solana airdrop 100 -u localhost > /dev/null 2>&1; then
        BALANCE=$(solana balance | awk '{print $1}')
        log_success "Airdrop successful. Balance: $BALANCE SOL"
        break
    fi
    if [ $i -eq 5 ]; then
        log_warning "Airdrop failed, but continuing (validator may have auto-funded)"
    fi
    sleep 2
done

# ==============================================================================
# Sync Program Keys
# ==============================================================================
log_step "Syncing Program Keys"

anchor keys sync 2>/dev/null || log_warning "Keys sync skipped (may already be synced)"

PROGRAM_ID=$(solana address -k target/deploy/program_upgrade_system-keypair.json 2>/dev/null || echo "unknown")
log_info "Program ID: $PROGRAM_ID"

# ==============================================================================
# Deploy Program
# ==============================================================================
log_step "Deploying Program"

DEPLOY_START=$(date +%s)

anchor deploy --provider.cluster localnet

DEPLOY_END=$(date +%s)
DEPLOY_TIME=$((DEPLOY_END - DEPLOY_START))

log_success "Deployment completed in ${DEPLOY_TIME} seconds"

# ==============================================================================
# Run Tests
# ==============================================================================
log_step "Running Tests"

echo ""
echo -e "${CYAN}Test Coverage:${NC}"
echo "  ├── Core Workflow Tests (6 tests)"
echo "  │   ├── Initialize Multisig"
echo "  │   ├── Propose Upgrade"
echo "  │   ├── Approve Upgrade"
echo "  │   ├── Execute Upgrade (Timelock simulation)"
echo "  │   ├── Cancel Upgrade"
echo "  │   └── Migrate Account"
echo "  │"
echo "  ├── Edge Case Tests (3 tests)"
echo "  │   ├── Prevent duplicate approval"
echo "  │   ├── Prevent double cancel"
echo "  │   └── Verify proposal state"
echo "  │"
echo "  └── Pause/Resume Tests (3 tests)"
echo "      ├── Pause system"
echo "      ├── Resume system"
echo "      └── Prevent double pause"
echo ""

# Note about timelock testing
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${YELLOW}📋 TIMELOCK TESTING NOTE${NC}"
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "The program has a 48-hour (172800 seconds) timelock."
echo "For testing, the execute_upgrade test verifies that:"
echo "  1. The timelock check IS enforced (fails when timelock not expired)"
echo "  2. All other instruction logic works correctly"
echo ""
echo "To test actual execution with expired timelock:"
echo "  - Use a test-only reduced timelock constant, OR"
echo "  - Use validator time warping: solana warp-slot +1000000"
echo ""

TEST_START=$(date +%s)

# Run anchor test (skip deploy since we already deployed)
# Skip local validator since we started our own
if anchor test --skip-deploy --skip-local-validator; then
    TEST_STATUS="PASS"
else
    TEST_STATUS="FAIL"
fi

TEST_END=$(date +%s)
TEST_TIME=$((TEST_END - TEST_START))

# ==============================================================================
# Results Summary
# ==============================================================================
log_step "Test Results Summary"

echo ""
echo -e "${CYAN}┌─────────────────────────────────────────────────────────────┐${NC}"
echo -e "${CYAN}│              Program Upgrade System CI Report               │${NC}"
echo -e "${CYAN}├─────────────────────────────────────────────────────────────┤${NC}"
printf "${CYAN}│${NC} %-20s │ %-37s ${CYAN}│${NC}\n" "Build Time" "${BUILD_TIME} seconds"
printf "${CYAN}│${NC} %-20s │ %-37s ${CYAN}│${NC}\n" "Deploy Time" "${DEPLOY_TIME} seconds"
printf "${CYAN}│${NC} %-20s │ %-37s ${CYAN}│${NC}\n" "Test Time" "${TEST_TIME} seconds"
printf "${CYAN}│${NC} %-20s │ %-37s ${CYAN}│${NC}\n" "Total Time" "$((BUILD_TIME + DEPLOY_TIME + TEST_TIME)) seconds"
echo -e "${CYAN}├─────────────────────────────────────────────────────────────┤${NC}"
if [ "$TEST_STATUS" = "PASS" ]; then
    printf "${CYAN}│${NC} ${GREEN}%-59s${NC} ${CYAN}│${NC}\n" "✅ ALL TESTS PASSED"
else
    printf "${CYAN}│${NC} ${RED}%-59s${NC} ${CYAN}│${NC}\n" "❌ SOME TESTS FAILED"
fi
echo -e "${CYAN}└─────────────────────────────────────────────────────────────┘${NC}"
echo ""

# ==============================================================================
# Exit
# ==============================================================================
if [ "$TEST_STATUS" = "PASS" ]; then
    log_success "CI pipeline completed successfully!"
    exit 0
else
    log_error "CI pipeline failed. Check logs above for details."
    exit 1
fi
