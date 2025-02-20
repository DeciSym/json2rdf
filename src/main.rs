// Copyright (c) 2024-2025, DeciSym, LLC
// Licensed under either of:
// - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// - BSD 3-Clause License (https://opensource.org/licenses/BSD-3-Clause)
// at your option.

//! # JSON2RDF Converter
//!
//! This is a Rust-based tool that converts JSON data into RDF format. It uses the `serde_json` crate
//! for JSON parsing and the `oxrdf` crate to construct RDF graphs.
//!
//! ## Features
//! - Parses JSON input and converts it to RDF triples
//! - Supports specifying a custom namespace for generated RDF nodes
//! - Outputs RDF data to a specified file
//!
//! ## Usage
//! Run the JSON2RDF converter from the command line. For detailed usage information, run:
//! ```
//! json2rdf --help
//! ```
//!
//! ## Example
//! To convert a JSON file to RDF format with a specified namespace and output file:
//! ```
//! json2rdf convert --namespace http://example.com/ns# --json-files data.json --output-file output.nt
//! ```
//! This will take `data.json`, apply the specified namespace, and save the RDF output in `output.nt`.
use clap::{Parser, Subcommand};
use json2rdf::*;

/// Command-line interface for JSON2RDF Converter
///
/// This struct defines the command-line interface (CLI) for interacting with the JSON2RDF converter.
#[derive(Parser)]
#[command(version, about = "Converts JSON data into RDF format.")]
struct Cli {
    /// CLI command selection
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Supported Commands
///
/// Contains the available commands for the JSON2RDF converter.
#[derive(Subcommand)]
enum Commands {
    /// Convert JSON to RDF format.
    ///
    /// The `convert` command parses a JSON file, converts it to RDF triples using `serde_json` for parsing
    /// and `oxrdf` to construct the graph, and saves the output.
    Convert {
        /// Namespace for RDF graph generation.
        ///
        /// A custom namespace to prefix RDF resources created from JSON keys and values.
        #[arg(short, long)]
        namespace: Option<String>,

        /// Path to input JSON file(s).
        ///
        /// Provide the path to one or more JSON files that will be parsed and converted.
        #[arg(short, long)]
        json_files: String,

        /// Path to output file.
        ///
        /// Optional: Specify the path to save the generated RDF data.
        #[arg(short, long)]
        output_file: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    // Match command and execute corresponding functionality
    match &cli.command {
        Some(Commands::Convert {
            namespace,
            json_files,
            output_file,
        }) => match json_to_rdf(json_files, namespace, output_file) {
            Ok(_) => {}
            Err(e) => eprintln!("Error writing: {}", e),
        },
        None => {}
    }
}
