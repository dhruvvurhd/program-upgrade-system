-- Test SQL to demonstrate the database schema works
-- This shows how the system would track a real upgrade proposal

-- Insert a test upgrade proposal
INSERT INTO upgrade_proposals (
    id,
    proposer,
    program,
    new_buffer,
    description,
    status,
    approval_count,
    timelock_until
) VALUES (
    gen_random_uuid(),
    '7xK9Q2vPvXfTc8mR3nY4bW1sL5pD8hF6jN2aT9cU4eV3',  -- Fake Solana pubkey
    '2BJdKmg9A2eyFgKi9zCHbd2zEoi1EhLu1fXDXR4mQet4',  -- Program ID
    '8x7Hf2vKjPmL3nY5bW2sL6pD9hF7jN3aT8cU5eV4wQ1',  -- Buffer address
    'Add new liquidation logic and improve margin calculations',
    'Pending',
    0,
    NOW() + INTERVAL '48 hours'
);

-- Get the proposal ID for subsequent inserts
DO $$
DECLARE
    proposal_uuid UUID;
BEGIN
    SELECT id INTO proposal_uuid FROM upgrade_proposals ORDER BY proposed_at DESC LIMIT 1;
    
    -- Insert 3 approvals (simulating multisig)
    INSERT INTO approval_history (proposal_id, approver, signature) VALUES
        (proposal_uuid, '5xK8Q1vPvXfTc7mR2nY3bW0sL4pD7hF5jN1aT8cU3eV2', '3TxSignature1...'),
        (proposal_uuid, '6xK7Q3vPvXfTc9mR4nY5bW2sL6pD9hF7jN3aT0cU5eV4', '4TxSignature2...'),
        (proposal_uuid, '9xK4Q6vPvXfTc2mR7nY8bW5sL9pD2hF0jN6aT3cU8eV7', '5TxSignature3...');
    
    -- Update proposal status after 3 approvals
    UPDATE upgrade_proposals  
    SET 
        status = 'TimelockActive',
        approval_count = 3
    WHERE id = proposal_uuid;
    
    -- Create a migration job
    INSERT INTO migration_jobs (proposal_id, total_accounts, migrated_accounts, status) VALUES
        (proposal_uuid, 1500, 0, 'Pending');
        
END $$;

-- Verify the data
SELECT 'Upgrade Proposals:' as table_name;
SELECT id, status, approval_count, description FROM upgrade_proposals;

SELECT '' as spacing;
SELECT 'Approval History:' as table_name;
SELECT approver, approved_at FROM approval_history ORDER BY approved_at;

SELECT '' as spacing;  
SELECT 'Migration Jobs:' as table_name;
SELECT total_accounts, migrated_accounts, status FROM migration_jobs;
