<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/serde_yml/images/logos/serde_yml.webp"
alt="Serde YML logo" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->
# Serde YML: Seamless YAML Serialization for Rust

Serde YML is a Rust library that simplifies YAML serialization and deserialization using Serde. Effortlessly convert Rust types to YAML and vice versa. Supports custom structs, enums, and error handling.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

![Banner of Serde YML][banner]

[![Made With Rust][made-with-rust-badge]][13] [![Crates.io][crates-badge]][08] [![Lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][09] [![License][license-badge]][03] [![Codecov][codecov-badge]][14]

• [Website][01] • [Documentation][09] • [Report Bug][04] • [Request Feature][04] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

<!-- markdownlint-enable MD033 -->

![divider][divider]

## Overview

`Serde YML` is a robust Rust library that simplifies the serialization and deserialization of Rust data structures to and from YAML format using the widely-used Serde framework. With Serde YML, you can effortlessly convert your Rust types into YAML strings and vice versa, streamlining the process of storing, transmitting, and manipulating structured data.providing style guides for your library.

## Features

- Serialize Rust data structures to YAML format
- Deserialize YAML data into Rust types
- Support for custom structs and enums using Serde's derive macros
- Handling of YAML's `!tag` syntax for representing enum variants
- Direct access to YAML values through the `Value` type and related types
- Comprehensive error handling with `Error`, `Location`, and `Result` types
- Well-documented with examples and explanations

## Usage

Serde YML offers a straightforward and intuitive API for working with YAML data in Rust. Here's a quick example of how to serialize and deserialize a Rust type:

```shell
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

fn main() -> Result<(), serde_yml::Error> {
    let point = Point { x: 1.0, y: 2.0 };

    // Serialize to YAML
    let yaml = serde_yml::to_string(&point)?;
    assert_eq!(yaml, "x: 1.0\ny: 2.0\n");

    // Deserialize from YAML
    let deserialized_point: Point = serde_yml::from_str(&yaml)?;
    assert_eq!(point, deserialized_point);

    Ok(())
}
```

## Installation

To use Serde YML in your Rust project, add the following to your `Cargo.toml` file:

```shell
[dependencies]
serde_yml = "0.0.1"
```

## Semantic Versioning Policy

For transparency into our release cycle and in striving to maintain
backward compatibility, `serde_yml` follows [semantic versioning][07].

## License

The project is licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

- [Apache License, Version 2.0][02]
- [MIT license][03]

## Contribution

We welcome all people who want to contribute. Please see the
[contributing instructions][05] for more information.

Contributions in any form (issues, pull requests, etc.) to this project
must adhere to the [Rust's Code of Conduct][12].

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Acknowledgements

A big thank you to all the awesome contributors of [serde_yml][06] for their help
and support. A special thank you goes to [David Tolnay][16] and his work on
[Serde YAML][17] for inspiring this project.

[01]: https://serdeyml.com "Serde YML"
[02]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[03]: http://opensource.org/licenses/MIT "MIT license"
[04]: https://github.com/sebastienrousseau/serde_yml/issues "Issues"
[05]: https://github.com/sebastienrousseau/serde_yml/blob/main/CONTRIBUTING.md "Contributing"
[06]: https://github.com/sebastienrousseau/serde_yml/graphs/contributors "Contributors"
[07]: http://semver.org/ "Semantic Versioning"
[08]: https://crates.io/crates/serde_yml "Serde YML on crates.io"
[09]: https://docs.rs/serde_yml "Serde YML on docs.rs"
[10]: https://lib.rs/crates/serde_yml "Serde YML on lib.rs"
[11]: https://github.com/sebastienrousseau/serde_yml/actions "GitHub Actions"
[12]: https://www.rust-lang.org/policies/code-of-conduct "Rust's Code of Conduct"
[13]: https://www.rust-lang.org "The Rust Programming Language"
[14]: https://codecov.io/gh/sebastienrousseau/serde_yml "Codecov"
[15]: https://www.reddit.com/r/rust/ "Rust Reddit"
[16]: https://github.com/dtolnay "David Tolnay"
[17]: https://github.com/dtolnay/serde-yaml "Serde YAML"


[banner]: https://kura.pro/serde_yml/images/titles/title-serde_yml.svg "Serde YML Banner"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/serde_yml?style=for-the-badge&token=Q9KJ6XXL67 "Codecov Badge"
[crates-badge]: https://img.shields.io/crates/v/serde_yml.svg?style=for-the-badge "Crates.io Badge"
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/serde_yml.svg?style=for-the-badge "Docs.rs Badge"
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge "Lib.rs Badge"
[license-badge]: https://img.shields.io/crates/l/serde_yml.svg?style=for-the-badge "License Badge"
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust "Made With Rust Badge"
