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

use clap::Error;
use oxrdf::vocab::xsd;
use oxrdf::{BlankNode, Graph, Literal, NamedNodeRef, TripleRef};

use serde_json::{Deserializer, Value};
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

/// Converts JSON data to RDF format.
///
/// This function reads JSON data from the specified file, processes it into RDF triples,
/// and outputs the RDF graph. Users can specify a namespace to use for RDF predicates and
/// an output file for saving the generated RDF data.
///
/// # Arguments
/// - `file_path`: Path to the JSON file.
/// - `namespace`: Optional custom namespace for RDF predicates.
/// - `output_file`: Optional output file path for writing RDF data.
///
/// # Example
/// ```rust
/// use json2rdf::json_to_rdf;
///
/// json_to_rdf(&"tests/airplane.json".to_string(), &Some("http://example.com/ns#".to_string()), &Some("output.nt".to_string()));
/// ```

pub fn json_to_rdf(
    file_path: &String,
    namespace: &Option<String>,
    output_file: &Option<String>,
) -> Result<Graph, Error> {
    let rdf_namespace: String = if namespace.is_some() {
        namespace.clone().unwrap()
    } else {
        "https://decisym.ai/json2rdf/model".to_owned()
    };

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let stream = Deserializer::from_reader(reader).into_iter::<Value>();

    let mut graph = Graph::default(); // oxrdf Graph object

    let mut subject_stack: VecDeque<BlankNode> = VecDeque::new();
    let mut property: Option<String> = None;

    for value in stream {
        match value {
            Ok(Value::Object(obj)) => {
                let subject = BlankNode::default(); // Create a new blank node
                subject_stack.push_back(subject.clone());

                for (key, val) in obj {
                    property = Some(format!("{}/{}", rdf_namespace, key));
                    process_value(
                        &mut subject_stack,
                        &property,
                        val,
                        &mut graph,
                        &rdf_namespace,
                    );
                }

                subject_stack.pop_back();
            }
            Ok(Value::Array(arr)) => {
                for val in arr {
                    process_value(
                        &mut subject_stack,
                        &property,
                        val,
                        &mut graph,
                        &rdf_namespace.clone(),
                    );
                }
            }
            Ok(other) => {
                process_value(
                    &mut subject_stack,
                    &property,
                    other,
                    &mut graph,
                    &rdf_namespace.clone(),
                );
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
            }
        }
    }

    if let Some(output_path) = output_file {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_path)
            .expect("Error opening file");

        writeln!(file, "{}", graph).expect("Error writing json2rdf data to file");
        Ok(graph)
    } else {
        return Ok(graph);
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
/// - `namespace`: Namespace for generating predicate URIs.
///
/// # JSON Type to RDF Conversion
/// - **Object**: Creates a blank node and recursively processes key-value pairs.
/// - **Array**: Iterates over elements and processes each as a separate value.
/// - **String**: Converts to `xsd:string` literal.
/// - **Boolean**: Converts to `xsd:boolean` literal.
/// - **Number**: Converts to `xsd:int` or `xsd:float` literal based on value type.

fn process_value(
    subject_stack: &mut VecDeque<BlankNode>,
    property: &Option<String>,
    value: Value,
    graph: &mut Graph,
    namespace: &String,
) {
    if let Some(last_subject) = subject_stack.clone().back() {
        if let Some(prop) = property {
            match value {
                Value::Bool(b) => {
                    graph.insert(TripleRef::new(
                        subject_stack.back().unwrap(),
                        NamedNodeRef::new(prop.as_str()).unwrap(),
                        &Literal::new_typed_literal(b.to_string(), xsd::BOOLEAN),
                    ));
                }
                Value::Number(num) => {
                    let literal = if let Some(int) = num.as_i64() {
                        int.to_string()
                    } else if let Some(float) = num.as_f64() {
                        float.to_string()
                    } else {
                        return;
                    };
                    graph.insert(TripleRef::new(
                        subject_stack.back().unwrap(),
                        NamedNodeRef::new(prop.as_str()).unwrap(),
                        &Literal::new_typed_literal(literal, xsd::INT),
                    ));
                }
                Value::String(s) => {
                    graph.insert(TripleRef::new(
                        subject_stack.back().unwrap(),
                        NamedNodeRef::new(prop.as_str()).unwrap(),
                        &Literal::new_typed_literal(s, xsd::STRING),
                    ));
                }
                Value::Null => {
                    //println!("Null value");
                }
                Value::Object(obj) => {
                    let subject = BlankNode::default();
                    subject_stack.push_back(subject);

                    graph.insert(TripleRef::new(
                        last_subject,
                        NamedNodeRef::new(prop.as_str()).unwrap(),
                        subject_stack.back().unwrap(),
                    ));

                    for (key, val) in obj {
                        let nested_property: Option<String> =
                            Some(format!("{}/{}/", namespace, key));
                        process_value(subject_stack, &nested_property, val, graph, namespace);
                    }
                    subject_stack.pop_back();
                }
                Value::Array(arr) => {
                    for val in arr {
                        process_value(subject_stack, property, val, graph, namespace);
                    }
                }
            }
        }
    }
}
