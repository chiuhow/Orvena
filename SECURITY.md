# Security Policy

Orvena is an early-stage open-source project. We take security seriously and
appreciate responsible disclosure.

## Reporting a vulnerability

Please **do not** open a public issue for security problems. Instead, use GitHub's
private vulnerability reporting on this repository
(**Security → Report a vulnerability**). Include:

- a description of the issue and its impact,
- steps to reproduce, and
- any relevant version / environment details.

We aim to acknowledge reports within a few days.

## Supported versions

Orvena is pre-1.0 and ships in small increments. Only the latest released version
is supported; fixes land on the main line.

## Handling of secrets

- **API keys live only in `.env`**, which is git-ignored. Orvena never writes keys to
  config or logs. Never commit real keys.
- The provider selection (not the key) is the only credential-adjacent value stored in
  config.

## Scope

Orvena executes a `verify` command you configure in `gates.yaml`, and writes files
within a task's declared scope. Treat the config you run as you would any executable
project configuration: review gate commands and scope before running against untrusted
input.
