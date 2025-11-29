# API Reference

Complete REST API documentation for the Program Upgrade & Migration System.

**Base URL**: `http://localhost:3000` (development) or `https://api.yourdomain.com` (production)

**Content-Type**: `application/json`

## Table of Contents

- [Authentication](#authentication)
- [Proposals](#proposals)
- [Migration](#migration)
- [Health](#health)
- [Error Handling](#error-handling)

## Authentication

Currently, authentication is handled via keypair paths in request bodies. Future versions will support API keys and OAuth.

## Proposals

### List All Proposals

Get all upgrade proposals with their current status.

```http
GET /proposals
```

**Response** `200 OK`:
```json
{
  "proposals": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "proposer": "7xK8...",
      "program": "2BJd...",
      "new_buffer": "8x7H...",
      "description": "Add liquidation improvements",
      "status": "Proposed",
      "approval_count": 1,
      "proposed_at": "2024-12-01T10:00:00Z",
      "timelock_until": null,
      "executed_at": null
    }
  ]
}
```

**Status Values**:
- `Proposed` - Created, awaiting approvals
- `Approved` - Threshold met, timelock not yet activated
- `TimelockActive` - Threshold met, waiting for timelock
- `Executed` - Successfully executed
- `Cancelled` - Cancelled before execution

---

### Get Proposal

Get details of a specific proposal.

```http
GET /proposals/:id
```

**Parameters**:
- `id` (UUID, required) - Proposal ID

**Response** `200 OK`:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "proposer": "7xK8...",
  "program": "2BJd...",
  "new_buffer": "8x7H...",
  "description": "Add liquidation improvements",
  "status": "TimelockActive",
  "approval_count": 3,
  "proposed_at": "2024-12-01T10:00:00Z",
  "timelock_until": "2024-12-03T10:00:00Z",
  "executed_at": null
}
```

**Error Responses**:
- `404 Not Found` - Proposal not found

---

### Create Proposal

Create a new upgrade proposal.

```http
POST /proposals/propose
```

**Request Body**:
```json
{
  "new_program_buffer": "8x7Hf2vKjP9qZ...",
  "description": "Add liquidation improvements and bug fixes"
}
```

**Fields**:
- `new_program_buffer` (string, required) - Solana buffer account address
- `description` (string, required) - Max 500 characters

**Response** `200 OK`:
```json
{
  "proposal_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "created"
}
```

**Error Responses**:
- `400 Bad Request` - Invalid buffer address or description too long
- `500 Internal Server Error` - Failed to create proposal

**Example**:
```bash
curl -X POST http://localhost:3000/proposals/propose \
  -H "Content-Type: application/json" \
  -d '{
    "new_program_buffer": "8x7Hf2vKjP9qZ...",
    "description": "Add liquidation improvements"
  }'
```

---

### Approve Proposal

Approve a proposal as a multisig member.

```http
POST /proposals/:id/approve
```

**Parameters**:
- `id` (UUID, required) - Proposal ID

**Request Body**:
```json
{
  "approver_keypair_path": "/path/to/keypair.json"
}
```

**Fields**:
- `approver_keypair_path` (string, required) - Path to keypair JSON file

**Response** `200 OK`:
```json
{
  "proposal_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "approved",
  "threshold_met": true,
  "timelock_activated": true,
  "timelock_expires_at": "2024-12-03T10:00:00Z"
}
```

**Fields**:
- `threshold_met` (boolean) - Whether approval threshold reached
- `timelock_activated` (boolean) - Whether 48h timelock started

**Error Responses**:
- `400 Bad Request` - Invalid keypair path
- `401 Unauthorized` - Not a multisig member
- `409 Conflict` - Already approved by this member
- `404 Not Found` - Proposal not found

**Example**:
```bash
curl -X POST http://localhost:3000/proposals/550e8400.../approve \
  -H "Content-Type: application/json" \
  -d '{"approver_keypair_path": "/Users/me/.config/solana/member1.json"}'
```

---

### Execute Upgrade

Execute an approved proposal after timelock expires.

```http
POST /proposals/:id/execute
```

**Parameters**:
- `id` (UUID, required) - Proposal ID

**Request Body**:
```json
{
  "executor_keypair_path": "/path/to/keypair.json"
}
```

**Response** `200 OK`:
```json
{
  "proposal_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "executed",
  "transaction_signature": "5J7x...",
  "executed_at": "2024-12-03T10:05:00Z"
}
```

**Error Responses**:
- `400 Bad Request` - Timelock not expired
- `403 Forbidden` - Insufficient approvals
- `404 Not Found` - Proposal not found
- `409 Conflict` - Already executed
- `500 Internal Server Error` - Execution failed

**Example**:
```bash
curl -X POST http://localhost:3000/proposals/550e8400.../execute \
  -H "Content-Type: application/json" \
  -d '{"executor_keypair_path": "/Users/me/.config/solana/id.json"}'
```

---

### Cancel Proposal

Cancel a proposal before execution (emergency only).

```http
POST /proposals/:id/cancel
```

**Parameters**:
- `id` (UUID, required) - Proposal ID

**Response** `200 OK`:
```json
{
  "proposal_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "cancelled"
}
```

**Error Responses**:
- `401 Unauthorized` - Not a multisig member
- `404 Not Found` - Proposal not found
- `409 Conflict` - Already executed (cannot cancel)

**Example**:
```bash
curl -X POST http://localhost:3000/proposals/550e8400.../cancel
```

---

## Migration

### Start Migration

Start a batch migration job for account state migration.

```http
POST /migration/start
```

**Request Body**:
```json
{
  "proposal_id": "550e8400-e29b-41d4-a716-446655440000",
  "account_addresses": [
    "7xK8f2vKjP9qZ...",
    "9mL2d3uHjN1pX...",
    "4pT5e6vGhM2kY..."
  ]
}
```

**Fields**:
- `proposal_id` (UUID, required) - Associated proposal
- `account_addresses` (array, required) - List of accounts to migrate

**Response** `200 OK`:
```json
{
  "job_id": "770f9500-f31c-52e5-b827-557766550111",
  "status": "started",
  "total_accounts": 1000,
  "estimated_duration_minutes": 120
}
```

**Error Responses**:
- `400 Bad Request` - Invalid account addresses
- `404 Not Found` - Proposal not found
- `500 Internal Server Error` - Failed to start migration

**Example**:
```bash
curl -X POST http://localhost:3000/migration/start \
  -H "Content-Type: application/json" \
  -d '{
    "proposal_id": "550e8400-e29b-41d4-a716-446655440000",
    "account_addresses": ["7xK8...", "9mL2..."]
  }'
```

---

### Get Migration Progress

Get the progress of a migration job.

```http
GET /migration/:id/progress
```

**Parameters**:
- `id` (UUID, required) - Migration job ID

**Response** `200 OK`:
```json
{
  "job_id": "770f9500-f31c-52e5-b827-557766550111",
  "total": 1000,
  "completed": 750,
  "percentage": 75.0,
  "status": "in_progress",
  "started_at": "2024-12-03T10:30:00Z",
  "estimated_completion": "2024-12-03T12:30:00Z",
  "errors": 5,
  "success_rate": 99.3
}
```

**Status Values**:
- `in_progress` - Currently migrating
- `completed` - All accounts migrated
- `failed` - Migration failed
- `paused` - Migration paused

**Error Responses**:
- `404 Not Found` - Migration job not found

**Example**:
```bash
curl http://localhost:3000/migration/770f9500.../progress
```

---

## Health

### Health Check

Check if the API is running and healthy.

```http
GET /health
```

**Response** `200 OK`:
```json
{
  "status": "healthy",
  "service": "upgrade-manager-backend",
  "version": "0.1.0",
  "timestamp": "2024-12-01T10:00:00Z",
  "database": "connected",
  "blockchain": "connected"
}
```

**Example**:
```bash
curl http://localhost:3000/health
```

---

## Error Handling

### Standard Error Response

All errors return a consistent format:

```json
{
  "error": {
    "code": "TIMELOCK_NOT_EXPIRED",
    "message": "Timelock period has not expired yet",
    "details": {
      "timelock_expires_at": "2024-12-03T10:00:00Z",
      "current_time": "2024-12-02T10:00:00Z",
      "hours_remaining": 24
    }
  }
}
```

### HTTP Status Codes

- `200 OK` - Request successful
- `400 Bad Request` - Invalid request parameters
- `401 Unauthorized` - Authentication required or failed
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - Resource not found
- `409 Conflict` - Resource conflict (e.g., already executed)
- `500 Internal Server Error` - Server error
- `503 Service Unavailable` - Service temporarily unavailable

### Common Error Codes

| Code | Description |
|------|-------------|
| `INVALID_BUFFER` | Invalid program buffer address |
| `TIMELOCK_NOT_EXPIRED` | Must wait for timelock period |
| `INSUFFICIENT_APPROVALS` | Need more multisig approvals |
| `UNAUTHORIZED_SIGNER` | Not a multisig member |
| `DUPLICATE_APPROVAL` | Already approved by this member |
| `PROPOSAL_ALREADY_EXECUTED` | Cannot modify executed proposal |
| `PROPOSAL_CANCELLED` | Proposal was cancelled |
| `MIGRATION_FAILED` | Account migration failed |
| `DATABASE_ERROR` | Database operation failed |
| `BLOCKCHAIN_ERROR` | Blockchain transaction failed |

---

## Rate Limiting

- **Default**: 100 requests per minute per IP
- **Burst**: Up to 200 requests
- **Headers**:
  - `X-RateLimit-Limit`: Max requests per minute
  - `X-RateLimit-Remaining`: Remaining requests
  - `X-RateLimit-Reset`: Time until limit resets

**Example Response** `429 Too Many Requests`:
```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Too many requests",
    "retry_after": 60
  }
}
```

---

## Webhooks (Future)

Subscribe to events:

- `proposal.created`
- `proposal.approved`
- `proposal.timelock_activated`
- `proposal.executed`
- `proposal.cancelled`
- `migration.started`
- `migration.completed`
- `migration.failed`

---

## SDK Examples

### JavaScript/TypeScript

```typescript
import axios from 'axios';

const api = axios.create({
  baseURL: 'http://localhost:3000',
  headers: { 'Content-Type': 'application/json' }
});

// Create proposal
const proposal = await api.post('/proposals/propose', {
  new_program_buffer: '8x7Hf2vKjP9qZ...',
  description: 'Add new feature'
});

// Approve proposal
await api.post(`/proposals/${proposal.data.proposal_id}/approve`, {
  approver_keypair_path: '/path/to/keypair.json'
});

// Check progress
const progress = await api.get(`/migration/${jobId}/progress`);
console.log(`Progress: ${progress.data.percentage}%`);
```

### Python

```python
import requests

API_URL = "http://localhost:3000"

# Create proposal
response = requests.post(f"{API_URL}/proposals/propose", json={
    "new_program_buffer": "8x7Hf2vKjP9qZ...",
    "description": "Add new feature"
})
proposal_id = response.json()["proposal_id"]

# Approve proposal
requests.post(f"{API_URL}/proposals/{proposal_id}/approve", json={
    "approver_keypair_path": "/path/to/keypair.json"
})

# Monitor migration
progress = requests.get(f"{API_URL}/migration/{job_id}/progress")
print(f"Progress: {progress.json()['percentage']}%")
```

### Rust

```rust
use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    // Create proposal
    let res = client
        .post("http://localhost:3000/proposals/propose")
        .json(&json!({
            "new_program_buffer": "8x7Hf2vKjP9qZ...",
            "description": "Add new feature"
        }))
        .send()
        .await?;
    
    let proposal: serde_json::Value = res.json().await?;
    println!("Created proposal: {}", proposal["proposal_id"]);
    
    Ok(())
}
```

---

## Testing

Test the API locally:

```bash
# Start local validator
solana-test-validator

# Deploy program
anchor deploy

# Start backend
cd backend && cargo run

# Test health endpoint
curl http://localhost:3000/health

# Test proposal creation
curl -X POST http://localhost:3000/proposals/propose \
  -H "Content-Type: application/json" \
  -d '{"new_program_buffer":"8x7...", "description":"Test"}'
```

---

## Support

- **Documentation**: https://docs.yourdomain.com
- **GitHub**: https://github.com/yourorg/upgrade-system
- **Discord**: https://discord.gg/yourserver
- **Email**: api-support@yourdomain.com
