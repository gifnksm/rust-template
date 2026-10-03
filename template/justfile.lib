# Public tasks for this repository.

# Format all crates in the workspace.
fmt *args:
    cargo fmt --all {{ args }}

# Run all compile checks.
check-all *args:
    just check-default-target {{ args }}
    just check-tests {{ args }}
    just check-benches {{ args }}

# Check default targets across exhaustive feature patterns.
check-default-target *args:
    cargo hack check --workspace --feature-powerset {{ args }}

# Check test targets across exhaustive feature patterns.
check-tests *args:
    cargo hack check --workspace --feature-powerset --tests {{ args }}

# Check benchmark targets across exhaustive feature patterns.
check-benches *args:
    cargo hack check --workspace --feature-powerset --benches {{ args }}

# Run all clippy checks.
clippy-all *args:
    just clippy-default-target {{ args }}
    just clippy-tests {{ args }}
    just clippy-benches {{ args }}

# Run clippy for default targets across exhaustive feature patterns.
clippy-default-target *args:
    cargo hack clippy --workspace --feature-powerset {{ args }}

# Run clippy for test targets across exhaustive feature patterns.
clippy-tests *args:
    cargo hack clippy --workspace --feature-powerset --tests {{ args }}

# Run clippy for benchmark targets across exhaustive feature patterns.
clippy-benches *args:
    cargo hack clippy --workspace --feature-powerset --benches {{ args }}

# Run tests across exhaustive feature patterns.
test-all *args:
    cargo hack test --workspace --feature-powerset {{ args }}

# Build coverage report.
llvm-cov-all *args:
    # Internal note: intentionally uses --all-features (not --feature-powerset).
    # Reason: powerset-style repeated runs can overwrite codecov output.
    cargo llvm-cov --workspace --all-features {{ args }}

# Build docs across exhaustive feature patterns.
doc-all *args:
    cargo hack doc --workspace --feature-powerset {{ args }}

# Build docs.rs-compatible docs for all packages.
docs-rs-all *args:
    rustup run nightly cargo hack docs-rs {{ args }}

# Synchronize README snippets for all packages.
sync-rdme-all *args:
    cargo hack sync-rdme --toolchain nightly --workspace {{ args }}

# Detect unused dependencies.
machete *args:
    cargo machete {{ args }}

# Check workflow files.
actionlint *args:
    actionlint {{ args }}

# Check spelling of entire workspace.
typos *args:
    typos {{ args }}

# Lint markdown files.
rumdl *args:
    rumdl check {{ args }}

# Check EditorConfig compliance.
editorconfig *args:
    editorconfig-checker {{ args }}

# Format TOML files.
tombi-format *args:
    uvx tombi format {{ args }}

# Lint TOML files.
tombi-lint *args:
    uvx tombi lint {{ args }}

# Run all CI-equivalent checks.
ci: ci-lint-rustfmt ci-check-rust ci-lint-rust ci-lint-unused-deps ci-lint-workflow ci-lint-spelling ci-lint-markdown ci-lint-editorconfig ci-lint-toml ci-check-rustdoc ci-check-docs-rs ci-check-markdown ci-test ci-coverage

# CI: formatting must be clean.
ci-lint-rustfmt:
    just fmt --check

# CI: compile checks.
ci-check-rust:
    just check-all

# CI: clippy warnings are treated as errors.
[env("CARGO_BUILD_WARNINGS", "deny")]
ci-lint-rust:
    just clippy-all

# CI: rustdoc warnings are treated as errors.
[env("CARGO_BUILD_WARNINGS", "deny")]
ci-check-rustdoc:
    just doc-all --no-deps

# CI: docs.rs warnings are treated as errors.
[env("CARGO_BUILD_WARNINGS", "deny")]
ci-check-docs-rs:
    just docs-rs-all

# CI: generated markdown content must produce no diff.
ci-check-markdown:
    just sync-rdme-all --check

# CI: dependency hygiene.
ci-lint-unused-deps:
    just machete

# CI: check workflow files.
ci-lint-workflow:
    just actionlint

# CI: check spelling.
ci-lint-spelling:
    just typos

# CI: lint markdown files.
ci-lint-markdown:
    just rumdl

# CI: check EditorConfig compliance.
ci-lint-editorconfig *args:
    just editorconfig {{ args }}

# CI: TOML formatting and lint must be clean.
ci-lint-toml:
    just tombi-format --check
    just tombi-lint --error-on-warnings

# CI: test suite.
ci-test:
    just test-all

# CI: uploadable coverage artifact.
ci-coverage:
    just llvm-cov-all --codecov --output-path target/codecov.json

# Pre-release gate is equivalent to full CI.
pre-release:
    just sync-rdme-all --allow-dirty
    if [ -z "${GITHUB_ACTIONS:-}" ]; then just ci; fi
