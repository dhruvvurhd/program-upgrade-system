#!/bin/bash
set -e

echo "🔄 Starting account migration..."

PROPOSAL_ID="$1"
ACCOUNTS_FILE="${2:-./accounts.txt}"
API_URL="${API_URL:-http://localhost:3000}"

if [ -z "$PROPOSAL_ID" ]; then
    echo "Usage: ./migrate_accounts.sh <PROPOSAL_ID> [ACCOUNTS_FILE]"
    echo "Example: ./migrate_accounts.sh abc-123 ./accounts.txt"
    exit 1
fi

if [ ! -f "$ACCOUNTS_FILE" ]; then
    echo "❌ Accounts file not found: $ACCOUNTS_FILE"
    echo "Create a file with one account address per line"
    exit 1
fi

# Read accounts into JSON array
ACCOUNTS=$(cat "$ACCOUNTS_FILE" | jq -R -s -c 'split("\n") | map(select(length > 0))')

echo "Migrating accounts for proposal: $PROPOSAL_ID"
echo "Accounts to migrate: $(echo "$ACCOUNTS" | jq '. | length')"
echo ""

# Start migration
RESPONSE=$(curl -s -X POST "$API_URL/migration/start" \
    -H "Content-Type: application/json" \
    -d "{
        \"proposal_id\": \"$PROPOSAL_ID\",
        \"account_addresses\": $ACCOUNTS
    }")

echo "✅ Migration started"
echo "$RESPONSE" | jq '.'

JOB_ID=$(echo "$RESPONSE" | jq -r '.job_id')

echo ""
echo "Job ID: $JOB_ID"
echo "Monitor progress: GET /migration/$JOB_ID/progress"
