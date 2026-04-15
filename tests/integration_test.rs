// Copyright (c) 2024-2025, DeciSym, LLC
// Licensed under either of:
// - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// - BSD 3-Clause License (https://opensource.org/licenses/BSD-3-Clause)
// at your option.

use json2rdf::{json_to_rdf, Json2RdfError};
use oxrdfio::{RdfFormat, RdfParser};
use std::fs::{self, File};

#[test]
fn test_graph_triple_count() {
    let triple_count_string = json_to_rdf(&["tests/airplane.json"], None, None);

    assert!(triple_count_string.is_ok());
    assert_eq!(triple_count_string.unwrap().unwrap().len(), 23);
}

#[test]
fn test_graph_write() {
    let output = "out.nt";

    let res = json_to_rdf(&["tests/airplane.json"], None, Some(output));

    assert!(res.is_ok());
    assert!(res.unwrap().is_none());

    let f = File::open(output).expect("unable to open output file for result verification");
    let quads = RdfParser::from_format(RdfFormat::NTriples)
        .for_reader(f)
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse generated output file");

    assert_eq!(quads.len(), 23);
    let _ = fs::remove_file(output);
}

#[test]
fn test_graph_write_truncates_existing() {
    let output = "out_truncate.nt";

    // Pre-populate with junk to prove truncation happens.
    fs::write(output, "stale garbage\n").expect("unable to seed stale output");

    // Two writes in a row should not accumulate; final file should hold one run's worth.
    for _ in 0..2 {
        let res = json_to_rdf(&["tests/airplane.json"], None, Some(output));
        assert!(res.is_ok());
    }

    let f = File::open(output).expect("unable to open output file for result verification");
    let quads = RdfParser::from_format(RdfFormat::NTriples)
        .for_reader(f)
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse generated output file");

    assert_eq!(quads.len(), 23);
    let _ = fs::remove_file(output);
}

#[test]
fn test_root_array() {
    let graph = json_to_rdf(&["tests/root_array.json"], None, None)
        .expect("conversion failed")
        .expect("expected graph");
    assert_eq!(graph.len(), 2);
}

#[test]
fn test_root_primitive_errors() {
    let result = json_to_rdf(&["tests/root_primitive.json"], None, None);
    assert!(matches!(
        result,
        Err(Json2RdfError::UnsupportedRootValue { kind: "number" })
    ));
}

#[test]
fn test_ndjson_stream_isolated() {
    let graph = json_to_rdf(&["tests/ndjson.json"], None, None)
        .expect("conversion failed")
        .expect("expected graph");
    assert_eq!(graph.len(), 2);
}

#[test]
fn test_multi_file_merges_graphs() {
    // root_array.json → 2 triples; ndjson.json → 2 triples; merged = 4.
    let graph = json_to_rdf(&["tests/root_array.json", "tests/ndjson.json"], None, None)
        .expect("conversion failed")
        .expect("expected graph");
    assert_eq!(graph.len(), 4);
}

#[test]
fn test_large_integers_preserve_precision() {
    let graph = json_to_rdf(&["tests/large_int.json"], None, None)
        .expect("conversion failed")
        .expect("expected graph");
    let serialized = graph.to_string();
    assert!(
        serialized.contains("\"9223372036854775807\"^^<http://www.w3.org/2001/XMLSchema#integer>"),
        "i64::MAX should round-trip as xsd:integer, got:\n{}",
        serialized
    );
    assert!(
        serialized.contains("\"18446744073709551615\"^^<http://www.w3.org/2001/XMLSchema#integer>"),
        "u64::MAX should round-trip as xsd:integer (not xsd:double), got:\n{}",
        serialized
    );
}

#[test]
fn test_empty_file_returns_empty_graph() {
    let graph = json_to_rdf(&["tests/empty_file.json"], None, None)
        .expect("conversion failed")
        .expect("expected graph");
    assert_eq!(graph.len(), 0);
}

#[test]
fn test_empty_object_at_root_produces_no_triples() {
    let graph = json_to_rdf(&["tests/empty_object.json"], None, None)
        .expect("conversion failed")
        .expect("expected graph");
    assert_eq!(graph.len(), 0);
}

#[test]
fn test_malformed_json_returns_error() {
    let result = json_to_rdf(&["tests/malformed.json"], None, None);
    assert!(
        matches!(result, Err(Json2RdfError::Json(_))),
        "expected Json2RdfError::Json, got {:?}",
        result.err()
    );
}

#[test]
fn test_missing_file_returns_error() {
    let result = json_to_rdf(&["tests/does_not_exist.json"], None, None);
    assert!(
        matches!(result, Err(Json2RdfError::Io(_))),
        "expected Json2RdfError::Io, got {:?}",
        result.err()
    );
}

#[test]
fn test_invalid_iri_key_returns_error() {
    let result = json_to_rdf(&["tests/invalid_iri_key.json"], None, None);
    assert!(
        matches!(result, Err(Json2RdfError::InvalidIri { .. })),
        "expected Json2RdfError::InvalidIri, got {:?}",
        result.err()
    );
}

#[test]
fn test_unicode_key_in_iri_range() {
    let graph = json_to_rdf(&["tests/unicode_key.json"], None, None)
        .expect("conversion failed")
        .expect("expected graph");
    assert_eq!(graph.len(), 1);
    assert!(
        graph.to_string().contains("中文"),
        "expected unicode predicate in output"
    );
}

#[test]
fn test_hash_namespace_not_mangled() {
    let graph = json_to_rdf(
        &["tests/airplane.json"],
        Some("http://example.com/ns#"),
        None,
    )
    .expect("conversion failed")
    .expect("expected graph");

    let serialized = graph.to_string();
    assert!(
        serialized.contains("<http://example.com/ns#aircraft>"),
        "expected predicate to use hash namespace without injected '/', got:\n{}",
        serialized
    );
    assert!(
        !serialized.contains("<http://example.com/ns#/"),
        "hash namespace should not have '/' appended, got:\n{}",
        serialized
    );
}

#[test]
fn test_slash_namespace_no_double_slash() {
    let graph = json_to_rdf(
        &["tests/airplane.json"],
        Some("http://example.com/ns/"),
        None,
    )
    .expect("conversion failed")
    .expect("expected graph");

    let serialized = graph.to_string();
    assert!(
        !serialized.contains("<http://example.com/ns//"),
        "trailing-slash namespace should not double the slash, got:\n{}",
        serialized
    );
    assert!(
        serialized.contains("<http://example.com/ns/aircraft>"),
        "expected predicate to use slash namespace without double slash, got:\n{}",
        serialized
    );
}
