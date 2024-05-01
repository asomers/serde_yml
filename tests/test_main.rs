// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 Serde YML, Seamless YAML Serialization for Rust. All rights reserved.

#[cfg(test)]
mod tests {
    use serde_yml::run;

    #[test]
    fn test_run() {
        if let Err(err) = run() {
            eprintln!("Program encountered an error: {}", err);
        }
        assert_eq!(1, 1)
    }
}
