# Asante Trade Contracts

Soroban smart-contract workspace for Asante Trade financial services on Stellar.

This repository is intentionally contract-only. It is not a monorepo and does not contain web applications, SDKs, or backend services.

## Planned contract domains

- `contracts/savings` — savings accounts and deposits
- `contracts/escrow` — escrow and conditional settlement
- `contracts/loans` — loan lifecycle and repayment rules
- `contracts/payments` — recurring and automated payments

The folders are scaffolding for contributors. No production contract implementation is included yet.

## Repository layout

```text
asante-trade-contracts/
├── contracts/             # Soroban contract crates, one domain per folder
│   ├── savings/
│   ├── escrow/
│   ├── loans/
│   └── payments/
├── tests/                 # Integration and security tests
├── scripts/               # Local validation and deployment helpers
├── docs/                  # Architecture, security, and contract specifications
├── .github/               # CI and contribution templates
├── Cargo.toml             # Rust workspace manifest
├── rust-toolchain.toml    # Pinned Rust toolchain
├── CONTRIBUTING.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
├── LICENSE
└── CHANGELOG.md
```

## Getting started

Install Rust and the Soroban CLI, then run the repository checks from this directory:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
```

These commands are included as contributor standards. They may remain unavailable until the first contract crate is added.

## Principles

- Keep each contract domain isolated and auditable.
- Prefer explicit authorization and bounded state transitions.
- Never commit secrets, deployed addresses, or network credentials.
- Add tests before implementing financial behavior.
- Document storage layout and upgrade assumptions for every contract.

See `CONTRIBUTING.md` and `docs/CONTRACT_STANDARDS.md` before opening a pull request.

## License

MIT. See `LICENSE`.

## Repository

[github.com/Asante-Trade/asante-trade-contracts](https://github.com/Asante-Trade/asante-trade-contracts)
    
