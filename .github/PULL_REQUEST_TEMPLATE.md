## Summary

Describe the contract or documentation change.

## Domain

- [ ] Savings
- [ ] Escrow
- [ ] Loans
- [ ] Payments
- [ ] Documentation/tooling

## Validation

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`

## Security review

- [ ] Authorization paths reviewed
- [ ] Input validation and arithmetic reviewed
- [ ] Events and storage changes documented
- [ ] No secrets or deployment credentials included
