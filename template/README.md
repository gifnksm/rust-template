<!-- cargo-sync-rdme title -->
<!-- cargo-sync-rdme badge -->

{% if crate_type == "lib" -%}
<!-- cargo-sync-rdme rustdoc -->
{%- else -%}
{{ project-description }}

## Installation

There are multiple ways to install {{project-name}}.
Choose any one of the methods below that best suits your needs.

### Pre-built binaries

Executable binaries are available for download on the [GitHub Release page].

You can also install the binary with [`cargo-binstall`] command.

```console
# Install pre-built binary
$ cargo binstall {{project-name}}
```

[GitHub Release page]: https://github.com/{{gh-username}}/{{project-name}}/releases/
[`cargo-binstall`]: https://github.com/cargo-bins/cargo-binstall

### Build from source using Rust

To build {{project-name}} executable from the source, you must have the Rust toolchain installed.
To install the rust toolchain, follow [this guide](https://www.rust-lang.org/tools/install).

Once you have installed Rust, the following command can be used to build and install {{project-name}}:

```console
# Install released version
$ cargo install {{project-name}}

# Install latest version
$ cargo install --git https://github.com/{{gh-username}}/{{project-name}}.git {{ project-name }}
```

{%- endif %}

## Minimum supported Rust version (MSRV)

The minimum supported Rust version is **Rust {{rust-version}}**.
At least the last 3 versions of stable Rust are supported at any given time.

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
