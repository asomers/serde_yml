// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

#[cfg(test)]
mod tests {
    use serde_yml::macro_ascii;

    // Tests for the macro_ascii! macro when input contains non-ASCII text
    #[test]
    #[should_panic(expected = "Failed to generate ASCII art")]
    fn test_macro_ascii_error() {
        let input = "日本語"; // Non-ASCII text, will cause an error
        let _ = macro_ascii!(input);
    }
}
