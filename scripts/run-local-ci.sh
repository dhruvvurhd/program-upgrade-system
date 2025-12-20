#!/bin/bash
# Script to run CI locally for program-upgrade-system
# Similar pattern to GDX component CI runners

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Running Program Upgrade System CI pipeline (via act)${NC}"
echo ""

# Get script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Change to project root
cd "$PROJECT_ROOT"
echo "Working directory: $(pwd)"

# Detect Architecture
ARCH=$(uname -m)
OS=$(uname -s)

echo "Detected System: $OS $ARCH"

USE_EXTERNAL=false
RPC_URL="http://127.0.0.1:8899"

# If on Apple Silicon, use external validator
if [[ "$OS" == "Darwin" && "$ARCH" == "arm64" ]]; then
    echo -e "${YELLOW}⚠️  Apple Silicon (ARM64) detected.${NC}"
    echo "Running the validator inside Docker (x86_64) is unstable/slow via emulation."
    echo "Switching to External Validator mode (running validator on host)."
    USE_EXTERNAL=true
    
    # On Docker Desktop for Mac, host is accessible via host.docker.internal
    RPC_URL="http://host.docker.internal:8899"
fi

# Check if act is installed
if ! command -v act &> /dev/null; then
    echo -e "${RED}❌ Error: nektos-act is not installed${NC}"
    echo ""
    echo "Install it with one of these methods:"
    echo "  macOS:   brew install act"
    echo "  Linux:   curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash"
    echo "  Windows: choco install act-cli"
    echo ""
    echo "Or download from: https://github.com/nektos/act/releases"
    exit 1
fi

# Check if Docker is running
if ! docker info &> /dev/null; then
    echo -e "${RED}❌ Error: Docker is not running${NC}"
    echo "Please start Docker and try again"
    exit 1
fi

if [ "$USE_EXTERNAL" = true ]; then
    # Check if validator is running on host
    if ! pgrep -f solana-test-validator > /dev/null; then
        echo -e "${YELLOW}Validator not running on host.${NC}"
        echo "Starting validator on host..."
        
        # Check if solana is installed
        if ! command -v solana-test-validator &> /dev/null; then
            echo -e "${RED}❌ solana-test-validator not found on host!${NC}"
            echo "Please install Solana CLI:"
            echo "sh -c \"\$(curl -sSfL https://release.anza.xyz/v1.18.17/install)\""
            exit 1
        fi
        
        # Start validator in background
        nohup solana-test-validator --reset --quiet > validator-host.log 2>&1 &
        VALIDATOR_PID=$!
        echo "Started host validator (PID: $VALIDATOR_PID)"
        
        # Wait for it to be ready
        echo "Waiting for validator to be ready..."
        for i in {1..30}; do
            if solana cluster-version --url http://127.0.0.1:8899 > /dev/null 2>&1; then
                echo -e "${GREEN}✓ Host validator is ready!${NC}"
                break
            fi
            sleep 1
        done
    else
        echo -e "${GREEN}✓ Found running validator on host.${NC}"
    fi
    
    echo ""
    echo -e "${BLUE}Running act with External Validator configuration...${NC}"
    echo "RPC URL: $RPC_URL"
    
    # Create cache directory
    CACHE_DIR="$HOME/.cache/program-upgrade-ci"
    mkdir -p "$CACHE_DIR"
    
    act \
        --workflows .github/workflows/ci.yml \
        --job build-and-test \
        --container-architecture linux/amd64 \
        --container-options "--network=host -v $CACHE_DIR:/root/.cache" \
        --bind \
        --pull=false \
        --env USE_EXTERNAL_VALIDATOR=true \
        --env SOLANA_RPC_URL="$RPC_URL" \
        --env ANCHOR_WALLET="/root/.config/solana/id.json" \
        --env CARGO_NET_GIT_FETCH_WITH_CLI=true
        
else
    echo -e "${BLUE}Running standard act pipeline (Validator in Docker)...${NC}"
    
    echo -e "${GREEN}▶️  Starting CI pipeline...${NC}"
    echo ""
    echo -e "${YELLOW}Note:${NC} The solana-test-validator will run in the background"
    echo "      and persist across workflow steps, just like on GitHub Actions."
    echo ""
    
    # Create cache directory
    CACHE_DIR="$HOME/.cache/program-upgrade-ci"
    mkdir -p "$CACHE_DIR"
    
    # Run act with the CI workflow
    act \
        --workflows .github/workflows/ci.yml \
        --job build-and-test \
        --container-architecture linux/amd64 \
        --container-options "--network=host -v $CACHE_DIR:/root/.cache" \
        --bind \
        --verbose \
        --pull=false \
        --env CARGO_NET_GIT_FETCH_WITH_CLI=true

    echo ""
    echo -e "${GREEN}✅ CI pipeline completed!${NC}"
fi
