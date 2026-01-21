use anyhow::{anyhow, Context, Result};
use std::io;

fn main() {
    divan::main();
}

#[divan::bench]
fn create_error_from_string() -> anyhow::Error {
    anyhow!("An error occurred")
}

#[divan::bench]
fn create_error_with_formatting() -> anyhow::Error {
    let value = 42;
    anyhow!("An error occurred with value: {}", value)
}

#[divan::bench]
fn create_error_from_io_error() -> anyhow::Error {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    anyhow::Error::new(io_error)
}

#[divan::bench]
fn add_context() -> Result<()> {
    let result: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    result.context("Failed to read configuration file")
}

#[divan::bench]
fn add_context_with_closure() -> Result<()> {
    let filename = "config.json";
    let result: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    result.with_context(|| format!("Failed to read file: {}", filename))
}

#[divan::bench]
fn downcast_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let error = anyhow::Error::new(io_error);
    
    divan::black_box(error.downcast_ref::<io::Error>());
}

#[divan::bench]
fn error_chain_iteration() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let error: Result<()> = Err(io_error.into());
    let error = error.context("Failed to read config").unwrap_err();
    
    for cause in error.chain() {
        divan::black_box(cause);
    }
}

#[divan::bench]
fn format_error_display() -> String {
    let error = anyhow!("An error occurred");
    format!("{}", error)
}

#[divan::bench]
fn format_error_debug() -> String {
    let error = anyhow!("An error occurred");
    format!("{:?}", error)
}
