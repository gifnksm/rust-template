<!-- rumdl-disable MD063 -->
<!-- cargo-sync-rdme title [[ -->
# {{ project-name }}
<!-- cargo-sync-rdme ]] -->
<!-- rumdl-enable MD063 -->
<!-- cargo-sync-rdme badge -->

{% if crate_type == "lib" -%}
<!-- cargo-sync-rdme rustdoc -->
{%- else -%}
{{ project-description }}

## Installation

There are multiple ways to install {{project-name}}.
Choose any one of the methods below that best suits your needs.

### Prebuilt Binaries

Executable binaries are published on the [GitHub Release page].

Download the appropriate archive for your platform (Windows, macOS, Linux) and architecture (x86_64, aarch64) and extract the archive. The archive contains the {{project-name}} executable.

If you use [`cargo-binstall`], you can install {{project-name}} with the following command:

```console
# Install prebuilt binary
cargo binstall {{project-name}}
```

[GitHub Release page]: https://github.com/{{gh-username}}/{{project-name}}/releases/
[`cargo-binstall`]: https://github.com/cargo-bins/cargo-binstall

### Install from Source

To install {{project-name}} from source, the Rust toolchain must be installed on your system.
See [the Rust installation guide](https://www.rust-lang.org/tools/install) if you do not have Rust installed yet.

Then install either the latest released version or the current development version from the Git repository.

```console
# Install released version
cargo install {{project-name}}

# Install latest version
cargo install --git https://github.com/{{gh-username}}/{{project-name}}.git {{ project-name }}
```

{%- endif %}

## Minimum Supported Rust Version (MSRV)

The minimum supported Rust version is **Rust {{rust-version}}**.

While a crate is a pre-release status (0.x.x) it may have its MSRV bumped in a patch release.
Once a crate has reached 1.x, any MSRV bump will be accompanied by a new minor version.

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
