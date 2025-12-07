# API Reference

## Base URL
```
http://localhost:3000
```

## Endpoints

### Health Check
```
GET /health
```
Returns server health status.

---

### List Proposals
```
GET /proposals
```
**Response:**
```json
{
  "proposals": [
    {
      "id": "uuid",
      "proposer": "pubkey",
      "status": "Proposed|Approved|TimelockActive|Executed|Cancelled",
      "proposed_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

---

### Get Proposal
```
GET /proposals/:id
```

---

### Create Proposal
```
POST /proposals/propose
```
**Request:**
```json
{
  "new_program_buffer": "Pubkey",
  "description": "Upgrade to v2.0"
}
```

---

### Approve Proposal
```
POST /proposals/:id/approve
```
**Request:**
```json
{
  "approver": "Pubkey"
}
```

---

### Execute Upgrade
```
POST /proposals/:id/execute
```
Executes upgrade after timelock expires.

---

### Cancel Proposal
```
POST /proposals/:id/cancel
```

---

### Start Migration
```
POST /migration/start
```
**Request:**
```json
{
  "proposal_id": "uuid",
  "account_addresses": ["pubkey1", "pubkey2"]
}
```

---

### Get Migration Progress
```
GET /migration/:id/progress
```
**Response:**
```json
{
  "total_accounts": 100,
  "migrated_accounts": 45,
  "progress_percent": 45.0
}
```
