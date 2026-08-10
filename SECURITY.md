# Security Policy

Asante Trade Contracts handles financial logic and should be treated as security-sensitive software.

## Reporting a vulnerability

Do not open a public issue for a suspected vulnerability. Contact the repository maintainers privately with:

- affected contract and entry point
- reproduction steps or proof of concept
- impact assessment
- suggested mitigation, if available

Please allow maintainers time to investigate before public disclosure.

## Development rules

- Never commit secrets or private keys.
- Never deploy unreviewed contracts to a public network.
- Treat storage migrations and authorization changes as breaking changes.
- Record audit findings and remediation in the relevant pull request.
