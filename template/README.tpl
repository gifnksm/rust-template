# {{project-name}}

[![maintenance status: {{maintenance-status}}](https://img.shields.io/badge/maintenance-{{maintenance-status | replace: "-", "--"}}-yellowgreen.svg)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-badges-section)
[![license: MIT OR APACHE-2.0](https://img.shields.io/crates/l/{{project-name}}.svg)](#license)
[![crates.io](https://img.shields.io/crates/v/{{project-name}}.svg)](https://crates.io/crates/{{project-name}})
[![docs.rs](https://docs.rs/{{project-name}}/badge.svg)](https://docs.rs/{{project-name}}/)
[![rust {{rust-version}}+ badge](https://img.shields.io/badge/rust-{{rust-version}}+-93450a.svg)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
[![Rust CI](https://github.com/{{gh-username}}/{{project-name}}/actions/workflows/ci.yml/badge.svg)](https://github.com/{{gh-username}}/{{project-name}}/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/{{gh-username}}/{{project-name}}/graph/badge.svg)](https://codecov.io/gh/{{gh-username}}/{{project-name}})

{{ '{{readme}}' }}

## Minimum supported Rust version (MSRV)

The minimum supported Rust version is **Rust {{ rust-version }}**.
At least the last 3 versions of stable Rust are supported at any given time.

While a crate is pre-release status (0.x.x) it may have its MSRV bumped in a patch release.
Once a crate has reached 1.x, any MSRV bump will be accompanied with a new minor version.

## License

This project is licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
