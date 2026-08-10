# Contract Standards

Every contract crate should follow these conventions.

## Structure

```text
contracts/<domain>/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── contract.rs
│   ├── error.rs
│   ├── events.rs
│   └── storage.rs
└── test.rs
```

## Required documentation

Document the contract's purpose, actors, authorization model, storage keys, events, errors, upgrade strategy, and supported network assumptions.

## Security requirements

- Validate all caller-controlled inputs.
- Make authorization explicit for every state-changing entry point.
- Avoid unbounded loops and unbounded storage growth.
- Emit events for important state transitions.
- Use checked arithmetic and clear error types.
- Test unauthorized, replay, boundary, and paused-state behavior.
- Never store secrets on-chain.

## Domain isolation

Contracts should not import application or frontend code. Shared primitives should be small, reviewed, and placed in the contract crate that owns them until a genuine reuse case exists.
