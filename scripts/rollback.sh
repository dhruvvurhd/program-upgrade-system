#!/bin/bash
set -e

echo "⚠️  ROLLBACK: Reverting to previous program version"

PROPOSAL_ID="$1"
REASON="${2:-Manual rollback}"
API_URL="${API_URL:-http://localhost:3000}"

if [ -z "$PROPOSAL_ID" ]; then
    echo "Usage: ./rollback.sh <PROPOSAL_ID> [REASON]"
    echo "Example: ./rollback.sh abc-123 'Critical bug found'"
    exit 1
fi

echo "Proposal to rollback: $PROPOSAL_ID"
echo "Reason: $REASON"
echo ""
read -p "Are you sure you want to rollback? (yes/no): " CONFIRM

if [ "$CONFIRM" != "yes" ]; then
    echo "Rollback cancelled"
    exit 0
fi

# Cancel current proposal
echo "Cancelling current proposal..."
curl -s -X POST "$API_URL/proposals/$PROPOSAL_ID/cancel" | jq '.'

echo ""
echo "✅ Rollback initiated"
echo ""
echo "Manual steps:"
echo "1. Deploy old program version to buffer"
echo "2. Create new upgrade proposal with old version"
echo "3. Fast-track approval if emergency"
