#!/bin/bash
set -e

echo "📝 Creating upgrade proposal..."

# Configuration
BUFFER_ADDRESS="$1"
DESCRIPTION="$2"
API_URL="${API_URL:-http://localhost:3000}"

if [ -z "$BUFFER_ADDRESS" ]; then
    echo "Usage: ./propose_upgrade.sh <BUFFER_ADDRESS> <DESCRIPTION>"
    echo "Example: ./propose_upgrade.sh 8x7... 'Fix critical bug in liquidation logic'"
    exit 1
fi

if [ -z "$DESCRIPTION" ]; then
    DESCRIPTION="Program upgrade"
fi

echo "Buffer: $BUFFER_ADDRESS"
echo "Description: $DESCRIPTION"
echo ""

# Create proposal via API
RESPONSE=$(curl -s -X POST "$API_URL/proposals/propose" \
    -H "Content-Type: application/json" \
    -d "{
        \"new_program_buffer\": \"$BUFFER_ADDRESS\",
        \"description\": \"$DESCRIPTION\"
    }")

echo "✅ Proposal created"
echo "$RESPONSE" | jq '.'

PROPOSAL_ID=$(echo "$RESPONSE" | jq -r '.proposal_id')

echo ""
echo "Proposal ID: $PROPOSAL_ID"
echo ""
echo "Next steps:"
echo "1. Get multisig members to approve: POST /proposals/$PROPOSAL_ID/approve"
echo "2. Wait for timelock (48 hours)"
echo "3. Execute upgrade: POST /proposals/$PROPOSAL_ID/execute"
