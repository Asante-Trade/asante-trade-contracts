# Contributing

Thank you for contributing to Asante Trade Contracts.

## Scope

This repository contains Soroban smart contracts only. Keep web applications, APIs, SDKs, and product documentation in their respective repositories.

## Before opening a pull request

1. Read `docs/CONTRACT_STANDARDS.md`.
2. Create or update a domain under `contracts/`.
3. Add unit and integration tests under the contract crate or `tests/`.
4. Document authorization, storage, events, errors, and upgrade behavior.
5. Run formatting, checks, and tests locally.

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace
cargo test --workspace
```

## Pull requests

- Use a focused branch and explain the financial behavior changed.
- Include security considerations and test evidence.
- Do not include private keys, deployed addresses, or generated secrets.
- A contract change requires review from at least one maintainer familiar with Soroban security.

## Commit messages

Use short imperative messages, for example:

- `Add savings storage specification`
- `Test escrow authorization rules`
- `Document payment contract events`
