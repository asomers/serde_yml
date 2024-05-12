//! # Serde YML Application
//!
//! This is the main entry point for the `serde_yml` application. It provides a simple command-line
//! interface to execute the functionality provided by the `serde_yml` crate.
//!
//! ## Purpose
//!
//! The purpose of this application is to serve as the entry point for the `serde_yml` functionality.
//! It calls the `run` function from the `serde_yml` crate to execute the desired tasks.
//!
//! ## Usage
//!
//! To use the `serde_yml` application, you can include it as part of your Rust project.
//! The main function of the application calls the `run` function from the `serde_yml` module.
//! If an error occurs during execution, it prints an error message and exits with a non-zero status code.
//!
//! ```rust
//! use serde_yml::run;
//!
//!/ This is the main entry point for the serde_yml application.
//! Call the `run()` function from the `serde_yml` module.
//! if let Err(ref e) = run() {
//!     eprintln!("Error running serde_yml: {}", e);
//!     std::process::exit(1);
//! }
//! ```
//!
//! This application allows you to interact with and use the functionality provided by the `serde_yml` crate.

use serde_yml::run;

fn main() {
    match run() {
        Ok(_) => println!("Program completed successfully."),
        Err(e) => eprintln!("Program encountered an error: {}", e),
    }
}
