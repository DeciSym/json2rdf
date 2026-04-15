// Copyright (c) 2024-2025, DeciSym, LLC
// Licensed under either of:
// - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// - BSD 3-Clause License (https://opensource.org/licenses/BSD-3-Clause)
// at your option.

//! # JSON2RDF Converter Library
//!
//! This library provides functionality for converting JSON data into RDF format.
//! It uses `serde_json` for JSON parsing and `oxrdf` to build and manage RDF graphs.
//!
//! ## Overview
//! - Converts JSON data structures into RDF triples, generating a graph representation.
//! - Supports blank nodes for nested structures and maps JSON properties to RDF predicates.
//!
//! ## Features
//! - Handles JSON Objects, Arrays, Booleans, Numbers, and Strings as RDF triples.
//! - Allows specifying a custom RDF namespace for generated predicates and objects.
//! - Outputs the RDF data to a specified file or prints it to the console.

use oxrdf::vocab::xsd;
use oxrdf::{BlankNode, Graph, IriParseError, Literal, NamedNodeRef, TripleRef};

use serde_json::{Deserializer, Value};
use std::fs::File;
use std::io::{BufReader, Write};
use thiserror::Error;

/// Errors that can occur while converting JSON to RDF.
#[derive(Debug, Error)]
pub enum Json2RdfError {
    /// Failure opening, reading, or writing a file.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Failure parsing the input JSON.
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    /// A JSON key produced a string that is not a valid IRI.
    #[error("invalid IRI {iri:?} generated from JSON key: {source}")]
    InvalidIri {
        iri: String,
        #[source]
        source: IriParseError,
    },

    /// A root-level JSON value has no predicate context and cannot be converted to a triple.
    #[error("unsupported root-level JSON {kind}; root must be an object or array")]
    UnsupportedRootValue { kind: &'static str },
}

/// Converts JSON data to RDF format.
///
/// This function reads JSON data from the specified file, processes it into RDF triples,
/// and outputs the RDF graph. Users can specify a namespace to use for RDF predicates and
/// an output file for saving the generated RDF data.
///
/// # Arguments
/// - `file_paths`: One or more paths to input JSON files. All files are merged into a single graph.
/// - `namespace`: Optional custom namespace for RDF predicates.
/// - `output_file`: Optional output file path for writing RDF data.
///
/// # Errors
/// Returns [`Json2RdfError`] if any input file cannot be read, the JSON cannot be parsed,
/// the output file cannot be written, or a JSON key produces an invalid IRI.
///
/// # Example
/// ```rust
/// use json2rdf::json_to_rdf;
///
/// let graph = json_to_rdf(
///     &["tests/airplane.json"],
///     Some("http://example.com/ns#"),
///     None,
/// )
/// .expect("conversion failed")
/// .expect("expected a graph");
/// assert!(!graph.is_empty());
/// ```
pub fn json_to_rdf(
    file_paths: &[&str],
    namespace: Option<&str>,
    output_file: Option<&str>,
) -> Result<Option<Graph>, Json2RdfError> {
    let mut prefix: String = namespace
        .map(str::to_owned)
        .unwrap_or_else(|| "https://decisym.ai/json2rdf/model".to_owned());
    // Respect hash (`#`), slash (`/`), and colon (`:`) terminators; otherwise default to `/`.
    if !prefix.ends_with(['#', '/', ':']) {
        prefix.push('/');
    }

    let mut graph = Graph::default(); // oxrdf Graph object
    let mut subject_stack: Vec<BlankNode> = Vec::new();

    for path in file_paths {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let stream = Deserializer::from_reader(reader).into_iter::<Value>();
        for value in stream {
            process_top_level(&mut subject_stack, value?, &mut graph, &prefix)?;
        }
    }

    if let Some(output_path) = output_file {
        let mut file = File::create(output_path)?;
        writeln!(file, "{}", graph)?;
        Ok(None)
    } else {
        Ok(Some(graph))
    }
}

/// Processes a single top-level JSON value from the input stream.
///
/// Each top-level value is handled independently: streamed values (NDJSON) do not
/// share predicate state with each other. Root-level primitives have no predicate
/// context and are rejected with [`Json2RdfError::UnsupportedRootValue`].
fn process_top_level(
    subject_stack: &mut Vec<BlankNode>,
    value: Value,
    graph: &mut Graph,
    prefix: &str,
) -> Result<(), Json2RdfError> {
    match value {
        Value::Object(obj) => {
            let subject = BlankNode::default();
            subject_stack.push(subject);

            for (key, val) in obj {
                let property = Some(format!("{}{}", prefix, key));
                process_value(subject_stack, &property, val, graph, prefix)?;
            }

            subject_stack.pop();
            Ok(())
        }
        Value::Array(arr) => {
            for item in arr {
                process_top_level(subject_stack, item, graph, prefix)?;
            }
            Ok(())
        }
        Value::Bool(_) => Err(Json2RdfError::UnsupportedRootValue { kind: "boolean" }),
        Value::Number(_) => Err(Json2RdfError::UnsupportedRootValue { kind: "number" }),
        Value::String(_) => Err(Json2RdfError::UnsupportedRootValue { kind: "string" }),
        Value::Null => Err(Json2RdfError::UnsupportedRootValue { kind: "null" }),
    }
}

/// This function handles different JSON data types, converting each into RDF triples:
/// - JSON Objects create new blank nodes and recursively process nested values.
/// - JSON Arrays iterate over each element and process it as an individual value.
/// - JSON Booleans, Numbers, and Strings are converted to RDF literals.
///
/// # Recursion for Nested Structures
/// Recursion is used to handle deeply nested JSON structures, which may contain multiple
/// levels of objects or arrays. This recursive approach allows the function to "dive" into
/// each nested layer of a JSON structure, creating blank nodes for sub-objects and handling
/// them as new subjects within the RDF graph. As a result, each level of JSON data is
/// systematically transformed into RDF triples, regardless of complexity or depth.
///
/// # Arguments
/// - `subject_stack`: Stack of blank nodes representing subjects. Each nested level pushes a new subject to the stack.
/// - `property`: RDF predicate (property) associated with the JSON value.
/// - `value`: JSON value to process.
/// - `graph`: RDF graph where triples are added.
/// - `prefix`: Fully-prepared namespace prefix (already terminated with `#`, `/`, or `:`)
///   used to build predicate IRIs by direct concatenation with each JSON key.
///
/// # JSON Type to RDF Conversion
/// - **Object**: Creates a blank node and recursively processes key-value pairs.
/// - **Array**: Iterates over elements and processes each as a separate value.
/// - **String**: Converts to `xsd:string` literal.
/// - **Boolean**: Converts to `xsd:boolean` literal.
/// - **Number**: Converts to `xsd:integer` for whole numbers, `xsd:double` for floating-point values.
fn process_value(
    subject_stack: &mut Vec<BlankNode>,
    property: &Option<String>,
    value: Value,
    graph: &mut Graph,
    prefix: &str,
) -> Result<(), Json2RdfError> {
    let Some(last_subject) = subject_stack.last().cloned() else {
        return Ok(());
    };
    let Some(prop) = property else {
        return Ok(());
    };

    let predicate =
        NamedNodeRef::new(prop.as_str()).map_err(|source| Json2RdfError::InvalidIri {
            iri: prop.clone(),
            source,
        })?;

    match value {
        Value::Bool(b) => {
            graph.insert(TripleRef::new(
                &last_subject,
                predicate,
                &Literal::new_typed_literal(b.to_string(), xsd::BOOLEAN),
            ));
        }
        Value::Number(num) => {
            let datatype = if num.is_i64() || num.is_u64() {
                xsd::INTEGER
            } else {
                xsd::DOUBLE
            };
            graph.insert(TripleRef::new(
                &last_subject,
                predicate,
                &Literal::new_typed_literal(num.to_string(), datatype),
            ));
        }
        Value::String(s) => {
            graph.insert(TripleRef::new(
                &last_subject,
                predicate,
                &Literal::new_typed_literal(s, xsd::STRING),
            ));
        }
        Value::Null => {}
        Value::Object(obj) => {
            let new_subject = BlankNode::default();
            graph.insert(TripleRef::new(&last_subject, predicate, &new_subject));
            subject_stack.push(new_subject);

            for (key, val) in obj {
                let nested_property: Option<String> = Some(format!("{}{}", prefix, key));
                process_value(subject_stack, &nested_property, val, graph, prefix)?;
            }
            subject_stack.pop();
        }
        Value::Array(arr) => {
            for val in arr {
                process_value(subject_stack, property, val, graph, prefix)?;
            }
        }
    }
    Ok(())
}
