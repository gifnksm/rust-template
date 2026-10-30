# Rust Template

A [`cargo-generate`][cargo-generate] template for Rust binary and library
crates, with GitHub Actions workflows for development, security checks, and
releases.

See the generated examples: [binary crate][example-bin] and
[library crate][example-lib]. Both are automatically updated from this template
and checked in CI.

## Features

Both crate types include release automation, compatibility checks, and shared
development tasks. Binary projects also include packaging and distribution.

### Publishing and Distribution

* **Trusted publishing to crates.io.** Release workflows use OpenID Connect
  (OIDC) to obtain short-lived publishing credentials through
  [Trusted Publishing], without storing a long-lived crates.io API token in
  GitHub Secrets.
* **Verifiable GitHub releases.** Binary release archives receive
  [artifact attestations]. Release workflows support [immutable releases] to
  protect published assets and their associated tags when enabled in the
  repository settings.
* **Application packages.** Binary projects build archives for Linux, macOS, and
  Windows, with shell completions, man pages, third-party license notices, and
  support for [`cargo-binstall`][cargo-binstall]. Binaries embed dependency
  information through [`cargo-auditable`][cargo-auditable].

See the release workflows for [binary crates][release-bin] and
[library crates][release-lib], and the
[distribution build matrix][dist-workflow].

### Quality and Compatibility

* **Compatibility across platforms and features.** Compilation and tests run on
  stable Rust and the minimum supported Rust version (MSRV), on Linux, macOS,
  and Windows. [`cargo-hack`][cargo-hack] checks feature combinations, with
  additional checks for Clippy, rustdoc, and docs.rs builds. Weekly beta checks
  help catch upcoming Rust compatibility issues.
* **Dependency and workflow security.** [RustSec] and [OSV-Scanner] checks cover
  Rust dependencies and GitHub Actions. Daily audits also check the latest
  stable release. Actions are locked with [`gh actions-lock`][gh actions-lock],
  and release workflows disable cache access to reduce exposure to cache
  poisoning.
* **Project-wide checks.** CI checks formatting, dependency license and source
  policies, unused dependencies, and repository files, with coverage reports
  uploaded to [Codecov].

See the CI workflows for [binary crates][ci-bin] and [library crates][ci-lib],
and the scheduled audit workflows for [binary crates][audit-bin] and
[library crates][audit-lib].

### Development and Maintenance

* **Shared local and CI commands.** [`mise`][mise] manages development tool
  versions and provides tasks for checks, formatting, documentation, and
  releases.
* **Documentation that stays in sync.** [`cargo-sync-rdme`][cargo-sync-rdme]
  updates README badges and library documentation from Cargo metadata and Rust
  doc comments. CI checks that they stay in sync.
* **Maintenance conventions included.** [Renovate] configuration, contribution
  guidelines, issue and pull request templates, a changelog, and MIT/Apache-2.0
  licensing provide a starting point for ongoing maintenance.

## Setup

Generate the project locally, then configure GitHub and crates.io to enable the
release workflows. Complete both configurations before your
[first release](#releases).

### Install Prerequisites

1. Install [Rust via rustup][rustup], [`mise`][mise], [GitHub CLI], Git, Bash 5
   or newer, and [`jq`][jq]. On macOS, install a recent Bash rather than using
   the system Bash.
2. Prepare the Rust toolchains, install `cargo-generate`, authenticate with
   GitHub, and install the Actions lockfile extension:

   ```console
   rustup toolchain install stable --profile minimal
   rustup toolchain install nightly --profile minimal
   cargo install cargo-generate --locked
   gh auth login
   gh extension install github/gh-actions-lock
   ```

   Nightly is used for documentation synchronization and docs.rs checks. `mise`
   installs the other development tools as tasks need them.

### Generate a Project

1. Generate a binary crate and answer the prompts for your project's metadata:

   ```console
   cargo generate --git https://github.com/gifnksm/rust-template.git \
     template --bin --allow-commands
   ```

   Use `--lib` instead of `--bin` for a library. `--allow-commands` enables the
   template's setup hook.
2. In the generated project directory, prepare the tool lockfile and commit the
   generated files:

   ```console
   cd YOUR_PROJECT
   mise trust
   mise lock
   git branch -M main
   git add .
   git commit -m "Initial commit"
   ```

### Configure GitHub

1. Create an empty GitHub repository matching the owner and project name entered
   during generation, and push the project to its `main` branch.
2. Configure release protection:

   * Enable [immutable releases] in the repository settings.
   * Create a `crates-io` environment with required approval and a deployment
     policy allowing tags matching `v[0-9]*`.
   * Add branch and tag rulesets to protect `main` and release tags. Use
     `ci-complete` as the required CI status check, and allow release
     maintainers to create release tags.

3. Verify the settings with
   [`check-github-settings`](./scripts/check-github-settings). From a separate
   clone of **this template repository**, run:

   ```console
   git clone https://github.com/gifnksm/rust-template.git
   cd rust-template
   mise trust
   mise exec -- ./scripts/check-github-settings YOUR_OWNER YOUR_REPOSITORY
   ```

   The checker reports differences without changing settings.

> [!NOTE]
> The checker uses the template author's repository configuration. Adapt the
> [expected settings](./etc/) to your reviewers and permissions, particularly
> for team or organization repositories.

Enable [Renovate] and [Codecov] for the generated repository if you want
dependency update pull requests and hosted coverage reports.

### Configure Crates.io

Set up Trusted Publishing before your [first release](#releases) so that every
project release is published by GitHub Actions using OIDC.

For a new crate, first register its name by publishing an empty bootstrap
package at version `0.0.0` using a crates.io API token:

1. Outside the generated project, create a separate temporary project with the
   same crate name:

   ```console
   cargo new --lib --name YOUR_CRATE crates-io-bootstrap
   cd crates-io-bootstrap
   ```

2. In the temporary project's `Cargo.toml`, set `version = "0.0.0"` and fill in
   `description` and `license`.
3. Create a temporary crates.io API token, authenticate, preview the package,
   then publish and yank the bootstrap version:

   ```console
   cargo login
   cargo publish --dry-run
   cargo publish
   cargo yank --version 0.0.0
   ```

   The bootstrap version remains stored on crates.io after
   [`cargo yank`][cargo-yank], but is excluded from new dependency resolutions.
4. Add a [Trusted Publisher][Trusted Publishing] in the crate's settings on
   crates.io with these values:

   | Setting | Value |
   | --- | --- |
   | Repository owner | Your GitHub username or organization |
   | Repository name | Your generated project's repository name |
   | Workflow filename | `release.yml` |
   | Environment | `crates-io` |

5. Revoke the bootstrap API token on crates.io and run `cargo logout` to remove
   the saved local credential. Return to the generated project and follow
   [Releases](#releases), including for its first version.

> [!NOTE]
> This bootstrap procedure is needed because crates.io does not yet support
> pending publishers, which would allow Trusted Publishing before any version of
> a crate has been published. See
> [RFC 3691's future possibilities][pending-publishers-rfc] and
> [crates.io Discussion #13039][pending-publishers-discussion].

## Development

Run these commands from a generated project. See its `CONTRIBUTING.md` for
contribution guidelines.

| Command | Purpose |
| --- | --- |
| `mise run ci` | Run checks, lints, and tests used by CI |
| `mise run test:rust` | Run Rust tests |
| `mise run format:rust` | Format Rust code |
| `mise run sync:markdown` | Synchronize the README |
| `mise run build:dist` | Build a host-platform archive (binary projects) |
| `mise tasks ls` | List available tasks |

## Releases

Before your first release, complete [Configure GitHub](#configure-github) and
[Configure Crates.io](#configure-cratesio). Every release follows the same steps
below.

1. Record notable changes in `CHANGELOG.md`.
2. Preview the release from the generated project:

   ```console
   mise exec -- cargo release patch -vv
   ```

   Replace `patch` with the desired version increment or an explicit version.
   Omit it to release the current version in `Cargo.toml`, such as `0.1.0` for
   the first release.
3. Review the changelog changes and planned actions, then rerun with `--execute`
   and the same version arguments:

   ```console
   mise exec -- cargo release patch -vv --execute
   ```

4. Approve the `crates-io` deployment in GitHub Actions when requested, and wait
   for the release workflow to finish.

[`cargo-release`][cargo-release] updates the version and changelog, runs the
pre-release checks, and pushes the commit and tag. GitHub Actions checks the
project, builds and attests binary archives, publishes the crate using Trusted
Publishing, and publishes the GitHub release.

## License

Licensed under either [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE), at
your option.

This repository borrows many components from [rust-github/template].

[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[cargo-binstall]: https://github.com/cargo-bins/cargo-binstall
[cargo-auditable]: https://github.com/rust-secure-code/cargo-auditable
[cargo-hack]: https://github.com/taiki-e/cargo-hack
[cargo-sync-rdme]: https://github.com/gifnksm/cargo-sync-rdme
[cargo-release]: https://github.com/crate-ci/cargo-release
[RustSec]: https://github.com/rustsec/advisory-db
[OSV-Scanner]: https://github.com/google/osv-scanner
[gh actions-lock]: https://github.com/github/gh-actions-lock
[Codecov]: https://github.com/codecov/codecov-action
[Renovate]: https://github.com/renovatebot/renovate
[jq]: https://github.com/jqlang/jq
[example-bin]: https://github.com/gifnksm/rust-template-generated-bin
[example-lib]: https://github.com/gifnksm/rust-template-generated-lib
[Trusted Publishing]: https://crates.io/docs/trusted-publishing
[cargo-yank]: https://doc.rust-lang.org/cargo/commands/cargo-yank.html
[pending-publishers-rfc]: https://rust-lang.github.io/rfcs/3691-trusted-publishing-cratesio.html#future-possibilities
[pending-publishers-discussion]: https://github.com/rust-lang/crates.io/discussions/13039
[artifact attestations]: https://docs.github.com/en/actions/concepts/security/artifact-attestations
[immutable releases]: https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases
[mise]: https://mise.jdx.dev/
[rustup]: https://rustup.rs/
[GitHub CLI]: https://cli.github.com/
[release-bin]: ./template/.github/workflows/release.bin.yml
[release-lib]: ./template/.github/workflows/release.lib.yml
[dist-workflow]: ./template/.github/workflows/reusable-build-dist.bin.yml
[ci-bin]: ./template/.github/workflows/ci.bin.yml
[ci-lib]: ./template/.github/workflows/ci.lib.yml
[audit-bin]: ./template/.github/workflows/audit.bin.yml
[audit-lib]: ./template/.github/workflows/audit.lib.yml
[rust-github/template]: https://github.com/rust-github/template
