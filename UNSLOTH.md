# ruff-unsloth

ruff 0.6.9 with the formatting style used by [unslothai/unsloth](https://github.com/unslothai/unsloth), so that `ruff format` produces it directly instead of needing Python passes before and after it. Upstream ruff follows PEP 8 here and does not offer this as an option (astral-sh/ruff#13344).

It installs as the `ruff-unsloth` binary and the `ruff_unsloth` module, so it can sit next to a stock ruff install.

## Style changes against ruff 0.6.9

- Keyword arguments, unannotated parameter defaults (lambdas included) and class pattern keywords are written `name = value`. The spaced `=` is measured as one column, so line breaking is exactly what stock ruff 0.6.9 produces.
- A `def` with 3 or more parameters and any default is always split one parameter per line. Every other `def` ignores its magic trailing comma and is joined when it fits.
- A multi-line `assert` with no comment on its rows and no one-element tuple ignores its magic trailing commas when its rows, joined with single spaces, fit the line length.
- In a nested block of at most 3 statements, the blank line after an import is removed when only blank lines separate it from the next statement (no comment, no decorator).

Everything else is ruff 0.6.9 unchanged.

## Install

Wheels for Linux (x86_64, aarch64, musl x86_64), macOS (arm64, x86_64) and Windows (x86_64) are attached to each release, with a `SHA256SUMS` file.
