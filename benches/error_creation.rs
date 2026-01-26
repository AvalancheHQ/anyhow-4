use anyhow::{anyhow, Context, Result};
use std::io;

fn main() {
    divan::main();
}

/// Benchmark creating errors with the anyhow! macro
#[divan::bench]
fn create_error_simple() -> anyhow::Error {
    anyhow!("something went wrong")
}

/// Benchmark creating errors with formatting
#[divan::bench]
fn create_error_formatted() -> anyhow::Error {
    let value = 42;
    anyhow!("something went wrong with value: {}", value)
}

/// Benchmark converting from std::io::Error
#[divan::bench]
fn convert_io_error() -> anyhow::Error {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    anyhow::Error::from(io_err)
}

/// Benchmark adding context to errors
#[divan::bench]
fn add_context() -> Result<()> {
    let result: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    result.context("Failed to read config file")
}

/// Benchmark adding lazy context to errors
#[divan::bench]
fn add_lazy_context() -> Result<()> {
    let filename = "config.json";
    let result: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    result.with_context(|| format!("Failed to read config from {}", filename))
}

/// Benchmark downcasting errors
#[divan::bench]
fn downcast_error() -> bool {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = anyhow::Error::from(io_err);
    err.downcast_ref::<io::Error>().is_some()
}

/// Benchmark error chain iteration with context
#[divan::bench]
fn iterate_chain() -> usize {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let result: Result<(), io::Error> = Err(io_err);
    let err = result
.context("Failed to read config")
        .err()
        .unwrap();
    err.chain().count()
}

/// Benchmark error formatting (Display)
#[divan::bench]
fn format_error() -> String {
    let err = anyhow!("something went wrong");
    format!("{}", err)
}

/// Benchmark error debug formatting
#[divan::bench]
fn format_error_debug() -> String {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let result: Result<(), io::Error> = Err(io_err);
    let err = result.context("Failed to read config").err().unwrap();
    format!("{:?}", err)
}
