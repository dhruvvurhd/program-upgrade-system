# Governance Model

## Overview
The upgrade system uses multisig governance to prevent unilateral changes while enabling protocol evolution.

## Approval Flow

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   Propose    │────▶│   Approve    │────▶│   Timelock   │
│   Upgrade    │     │  (3 of 5)    │     │  (48 hours)  │
└──────────────┘     └──────────────┘     └──────────────┘
                                                 │
                                                 ▼
┌──────────────┐                         ┌──────────────┐
│   Cancel     │◀────────────────────────│   Execute    │
│  (Emergency) │                         │   Upgrade    │
└──────────────┘                         └──────────────┘
```

## Roles

### Multisig Members
- Can propose upgrades
- Can approve proposals
- Can cancel proposals (emergency)
- Can pause/resume system

### Threshold
Default: **3 of 5** members must approve before timelock activates.

## Timelock Period
- **Duration**: 48 hours minimum
- **Purpose**: Allows users to exit positions if they disagree with upgrade
- **Override**: Cannot be bypassed

## Emergency Procedures

### Pause System
Any multisig member can pause all operations:
```rust
pub fn pause_system(ctx: Context<PauseSystem>) -> Result<()>
```

### Cancel Upgrade
Before execution, any member can cancel:
```rust
pub fn cancel_upgrade(ctx: Context<CancelUpgrade>, proposal_id: Pubkey) -> Result<()>
```

## Status Transitions
| From | To | Trigger |
|------|----|---------|
| Proposed | Approved | Threshold met |
| Approved | TimelockActive | Automatic |
| TimelockActive | Executed | 48h elapsed + execute called |
| Any (pre-execute) | Cancelled | Cancel called |
