-- Upgrade Proposals Table
CREATE TABLE IF NOT EXISTS upgrade_proposals (
    id UUID PRIMARY KEY,
    proposer TEXT NOT NULL,
    program TEXT NOT NULL,
    new_buffer TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL,
    approval_count INTEGER NOT NULL DEFAULT 0,
    proposed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    timelock_until TIMESTAMPTZ,
    executed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Approval History Table
CREATE TABLE IF NOT EXISTS approval_history (
    id UUID PRIMARY KEY,
    proposal_id UUID NOT NULL REFERENCES upgrade_proposals(id) ON DELETE CASCADE,
    approver TEXT NOT NULL,
    approved_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Migration Jobs Table
CREATE TABLE IF NOT EXISTS migration_jobs (
    id UUID PRIMARY KEY,
    proposal_id UUID REFERENCES upgrade_proposals(id) ON DELETE SET NULL,
    total_accounts BIGINT NOT NULL,
    migrated_accounts BIGINT NOT NULL DEFAULT 0,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    finished_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Rollback Events Table
CREATE TABLE IF NOT EXISTS rollback_events (
    id UUID PRIMARY KEY,
    proposal_id UUID REFERENCES upgrade_proposals(id) ON DELETE SET NULL,
    executed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reason TEXT NOT NULL,
    executed_by TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Account Migrations Table
CREATE TABLE IF NOT EXISTS account_migrations (
    id UUID PRIMARY KEY,
    migration_job_id UUID REFERENCES migration_jobs(id) ON DELETE CASCADE,
    account_address TEXT NOT NULL,
    old_version INTEGER NOT NULL,
    new_version INTEGER NOT NULL,
    migrated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status TEXT NOT NULL,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_proposals_status ON upgrade_proposals(status);
CREATE INDEX IF NOT EXISTS idx_proposals_proposed_at ON upgrade_proposals(proposed_at DESC);
CREATE INDEX IF NOT EXISTS idx_approvals_proposal_id ON approval_history(proposal_id);
CREATE INDEX IF NOT EXISTS idx_migration_jobs_proposal_id ON migration_jobs(proposal_id);
CREATE INDEX IF NOT EXISTS idx_account_migrations_job_id ON account_migrations(migration_job_id);
