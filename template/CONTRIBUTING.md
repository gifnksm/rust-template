# Contribution guidelines

First off, thank you for considering contributing to {{project-name}}.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/{{gh-username}}/{{project-name}}/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Try to do one pull request per change.

Run `just ci` before opening or updating a pull request.

When a pull request resolves an issue, reference it in the PR description with
`Closes #<number>`. When it is related to an issue but does not resolve it,
reference it with `Refs #<number>`.

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/{{gh-username}}/{{project-name}}/blob/main/CHANGELOG.md)
file under the **Unreleased** section.

Add the pull request number to changelog entries when available. If the pull
request number is not known yet, update the changelog after creating the pull
request.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!

## Developing

### Set up

This is no different than other Rust projects.

```console
git clone https://github.com/{{gh-username}}/{{project-name}}
cd {{project-name}}
cargo test
```

### Useful Commands

- Run lint and static checks during development:

  ```console
  just ci-lint
  ```

  `just ci-lint` relies on additional tools such as `just`, `cargo-hack`,
  `cargo-machete`, `actionlint`, `typos`, and Node.js (providing `node`/`npx`
  for `markdownlint-cli`).

- Run the full CI-equivalent suite, including docs and tests, before opening or updating a pull request:

  ```console
  just ci
  ```
{%- if crate_type == "bin" %}

- Build and run release version:

  ```console
  cargo build --release && cargo run --release
  ```
{%- endif %}

- Run formatting checks:

  ```console
  just fmt --check
  ```

- Format the code in the project:

  ```console
  just fmt
  ```

- Run all tests:

  ```console
  just test-all
  ```

- Run Clippy:

  ```console
  just clippy-all -- -D warnings
  ```
