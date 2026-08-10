📜 Asante Trade Contracts

Soroban smart contracts powering the programmable financial layer of Asante Trade — savings, escrow, loans, and automated payments on the Stellar network.

Status: 🚧 Pre-implementation. This repository is currently scaffolding — folder structure, tooling, and contributor standards are in place, but no contract logic has landed yet. See Roadmap for what's built vs. planned. Nothing here has been audited; treat any claim to the contrary as out of date.

🌍 Overview

Asante Trade helps field agents deliver financial services — wallets, loans, savings, bill payments — to communities without access to traditional banking. This repository holds the on-chain layer: Soroban smart contracts that will automate the core financial primitives the platform depends on, so that funds movement, loan terms, and savings rules are enforced on-chain rather than trusted to a backend alone.

This repo is contract-only — no web app, SDK, or backend service lives here. See Related repositories for those.

🚀 Planned core features
Domain	What it will do	Status
Savings	Deposit accounts with configurable withdrawal and interest rules	📋 Not started
Escrow	Hold funds between two parties pending a condition, then release or refund	📋 Not started
Loans	Loan origination, repayment schedules, default/late handling	📋 Not started
Payments	Recurring and automated transfers (e.g. scheduled repayments)	📋 Not started
Access control	Multi-signature authorization and role-based permissions across contracts	📋 Not started
Asset transfers & verification	Stellar asset movement with on-chain transaction verification	📋 Not started
🧠 How the domains are meant to work
Savings — an account holder deposits an asset into a savings contract that enforces withdrawal rules (lock-up periods, minimum balances) and, eventually, interest accrual.
Escrow — two parties (e.g. an agent and a customer) lock funds against a condition; the contract releases to one side or refunds the other once that condition is met or disputed.
Loans — models the loan lifecycle on-chain: disbursement, an immutable repayment schedule, and rules for what happens on late or missed payments.
Payments — automates recurring transfers (like loan installments) without a human triggering each one manually.

These are design intentions, not shipped behavior — none of the above exists in code yet.

🗂️ Repository structure
asante-trade-contracts/
├── contracts/             # Soroban contract crates, one domain per folder
│   ├── savings/
│   ├── escrow/
│   ├── loans/
│   └── payments/
├── tests/                 # Integration and security tests
├── scripts/                # Local validation and deployment helpers
├── docs/                   # Architecture, security, and contract specifications
├── .github/                 # CI and contribution templates
├── Cargo.toml               # Rust workspace manifest
├── rust-toolchain.toml      # Pinned Rust toolchain
├── CONTRIBUTING.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
├── LICENSE
└── CHANGELOG.md
🛠️ Tech stack
Technology	Purpose
Rust	Smart contract language
Soroban SDK	Smart contract development
Stellar network	Settlement layer
Cargo	Build system
Soroban CLI / Stellar CLI	Deployment and testing
GitHub Actions	CI/CD (planned)
🚀 Development
Prerequisites
Rust (latest stable)
Cargo
Soroban CLI
bash
cargo install --locked soroban-cli
Building
bash
cargo build

Optimized WASM build:

bash
cargo build --release --target wasm32v1-none
Testing
bash
cargo test --workspace

These commands are the contributor standard for this repo, but will have little to check until the first contract crate lands — see Roadmap.

Deployment (example, once a contract exists)
bash
soroban contract deploy \
  --wasm target/wasm32v1-none/release/savings.wasm \
  --source <ADMIN_SECRET_KEY> \
  --network testnet

Deployment scripts and network configs will be added as contracts land.

🔐 Security

Security is the top priority for a repo that will eventually move real money. Before any mainnet deployment, every contract needs:

A comprehensive automated test suite
Explicit, bounded authorization on every state-changing call
An external security audit

None of this repo's contracts are audited, because none exist yet. If you discover a vulnerability once contracts do exist, report it privately per SECURITY.md — please don't open a public issue.

🧭 Design principles
Keep each contract domain isolated and independently auditable.
Prefer explicit authorization and bounded state transitions over implicit trust.
Never commit secrets, deployed contract addresses, or network credentials.
Write tests before implementing financial behavior, not after.
Document storage layout and upgrade assumptions for every contract as it's built.
🧩 Contributing
Review open issues — for anything beyond a small fix, open an issue first so effort isn't duplicated.
Follow Rust and Soroban best practices; include tests with any new functionality.
Read CONTRIBUTING.md and docs/CONTRACT_STANDARDS.md before opening a PR.
🗺️ Roadmap
 Savings contract
 Escrow contract
 Loan contract
 Automated payment contract
 Multi-signature wallet support
 Comprehensive test suite
 External security audit
 Testnet deployment
 Mainnet deployment
🔗 Related repositories
Repository	Description
asante-trade-web	Frontend dashboards and customer portals
asante-trade-backend	Backend APIs and business logic
asante-trade-mobile-agent	Mobile app for field agents (planned)
🎯 Vision

Asante Trade Contracts aim to be the decentralized foundation for programmable financial services inside the Asante Trade ecosystem — using Soroban on Stellar to make savings, escrow, and lending transparent and auditable by design, in service of financial inclusion across Africa.

📄 License

MIT — see LICENSE.
