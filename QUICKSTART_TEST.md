# Quick Start Testing Guide

## 🚀 Fast Track - Get Running in 10 Minutes

### Step 1: Start Local Blockchain (Terminal 1)
```bash
solana-test-validator
```
**Leave this running!**

### Step 2: Configure & Deploy (Terminal 2)
```bash
cd /Users/dhruvmishra/UPGRADECPI/program-upgrade-system

# Set to localhost
solana config set --url localhost

# Get some SOL
solana airdrop 10

# Build and deploy
anchor build
anchor deploy
```

**✅ Success check**: You should see `Program Id: <some_address>`

**📝 Copy this Program ID** - you'll need it!

---

### Step 3: Setup Database (Same Terminal)
```bash
# Create database (if not exists)
createdb upgrade_manager 2>/dev/null || true

# Load schema
psql upgrade_manager < backend/src/db/schema.sql
```

**✅ Success check**: Should see `CREATE TABLE` messages

---

### Step 4: Configure Backend
```bash
cd backend

# Create config file
cat > .env <<EOF
DATABASE_URL=postgresql://localhost/upgrade_manager
SOLANA_RPC_URL=http://localhost:8899
PROGRAM_ID=<PASTE_YOUR_PROGRAM_ID_HERE>
PORT=3000
EOF
```

**⚠️ Replace `<PASTE_YOUR_PROGRAM_ID_HERE>` with your actual Program ID from Step 2!**

---

### Step 5: Start Backend (Terminal 3)
```bash
cd /Users/dhruvmishra/UPGRADECPI/program-upgrade-system/backend
cargo run
```

**✅ Success check**: 
```
Server listening on 0.0.0.0:3000
Connected to database
```

---

### Step 6: Create Test Members
```bash
# In Terminal 2
cd /Users/dhruvmishra/UPGRADECPI/program-upgrade-system

# Create 5 voting members
mkdir -p test-keys
solana-keygen new -o test-keys/member1.json --no-bip39-passphrase
solana-keygen new -o test-keys/member2.json --no-bip39-passphrase
solana-keygen new -o test-keys/member3.json --no-bip39-passphrase
solana-keygen new -o test-keys/member4.json --no-bip39-passphrase
solana-keygen new -o test-keys/member5.json --no-bip39-passphrase

# Fund them
solana airdrop 1 test-keys/member1.json
solana airdrop 1 test-keys/member2.json
solana airdrop 1 test-keys/member3.json
```

---

### Step 7: Test the System

#### 7.1 Check Backend is Running
```bash
curl http://localhost:3000/proposals
```

**Expected**: `[]` (empty array - no proposals yet)

#### 7.2 Make a Code Change
```bash
# Edit the program slightly
nano programs/program-upgrade-system/src/lib.rs
# Add a comment like: // Test upgrade v2

# Save and rebuild
anchor build
```

#### 7.3 Create Program Buffer
```bash
# This uploads your new code to Solana
solana program write-buffer \
  target/deploy/program_upgrade_system.so
```

**📝 Copy the Buffer Address** that gets printed!

#### 7.4 Create Proposal
```bash
curl -X POST http://localhost:3000/proposals \
  -H "Content-Type: application/json" \
  -d '{
    "new_program_buffer": "<PASTE_BUFFER_ADDRESS_HERE>",
    "description": "My first test upgrade"
  }'
```

**Expected response**:
```json
{
  "proposal_id": "some-uuid-here",
  "status": "Pending"
}
```

**📝 Copy the proposal_id!**

#### 7.5 Approve Proposal (3x)
```bash
# Replace <PROPOSAL_ID> with your actual proposal ID

# Approval 1
curl -X POST http://localhost:3000/proposals/<PROPOSAL_ID>/approve \
  -H "Content-Type: application/json" \
  -d '{"approver_keypair_path": "test-keys/member1.json"}'

# Approval 2
curl -X POST http://localhost:3000/proposals/<PROPOSAL_ID>/approve \
  -H "Content-Type: application/json" \
  -d '{"approver_keypair_path": "test-keys/member2.json"}'

# Approval 3 (triggers timelock!)
curl -X POST http://localhost:3000/proposals/<PROPOSAL_ID>/approve \
  -H "Content-Type: application/json" \
  -d '{"approver_keypair_path": "test-keys/member3.json"}'
```

**After the 3rd approval**, you should see:
```json
{
  "status": "TimelockActive",
  "timelock_until": "2024-12-01T..."
}
```

**🎉 SUCCESS!** Your timelock is activated!

#### 7.6 Verify in Database
```bash
psql upgrade_manager -c "
SELECT 
  id, 
  status, 
  approval_count,
  timelock_until 
FROM upgrade_proposals;
"
```

You should see your proposal with:
- `status`: TimelockActive
- `approval_count`: 3
- `timelock_until`: 48 hours from now

#### 7.7 Check Approval History
```bash
psql upgrade_manager -c "
SELECT 
  approver, 
  approved_at 
FROM approval_history;
"
```

You should see 3 rows (your 3 approvals)!

---

## 🎯 What You Just Proved

✅ **Your smart contract works** - It accepted proposals and approvals  
✅ **Multisig works** - Required 3 approvals to activate timelock  
✅ **Backend works** - API handled all requests  
✅ **Database works** - All actions were logged  
✅ **Timelock works** - Status changed after 3rd approval  

---

## 🐛 If Something Fails

### "Connection refused" on port 3000
- Check backend is running: Look at Terminal 3
- Restart: `cargo run` in backend folder

### "Program not found"
- Redeploy: `anchor deploy`
- Check: `solana program show <PROGRAM_ID>`

### "Database error"
- Check PostgreSQL is running: `psql -l`
- Recreate: `dropdb upgrade_manager && createdb upgrade_manager`
- Reload schema: `psql upgrade_manager < backend/src/db/schema.sql`

### "TransactionError"
- Check SOL balance: `solana balance`
- Airdrop more: `solana airdrop 5`

---

## 📚 Next Steps

1. ✅ **You've completed basic testing!**
2. Try cancelling a proposal
3. Try executing after timelock (requires waiting or modifying timelock duration)
4. Read the full beginner walkthrough for deeper understanding
5. Check out the architecture docs in `docs/`

---

## 💡 Pro Tips

- Keep Terminal 1 (validator) running always
- Watch Terminal 3 (backend logs) to see what's happening
- Use `psql upgrade_manager` to inspect database anytime
- Use `solana logs` to see blockchain events

**You built something amazing! 🎉**
