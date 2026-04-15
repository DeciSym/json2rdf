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
    let triple_count_string = json_to_rdf(&"tests/airplane.json".to_string(), &None, &None);

    assert!(triple_count_string.is_ok());
    assert_eq!(triple_count_string.unwrap().unwrap().len(), 23);
}

#[test]
fn test_graph_write() {
    let output = "out.nt".to_string();

    let res = json_to_rdf(
        &"tests/airplane.json".to_string(),
        &None,
        &Some(output.clone()),
    );

    assert!(res.is_ok());
    assert!(res.unwrap().is_none());

    let f = File::open(output.clone()).expect("unable to open output file for result verification");
    let quads = RdfParser::from_format(RdfFormat::NTriples)
        .for_reader(f)
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse generated output file");

    assert_eq!(quads.len(), 23);
    let _ = fs::remove_file(output.clone());
}

#[test]
fn test_graph_write_truncates_existing() {
    let output = "out_truncate.nt".to_string();

    // Pre-populate with junk to prove truncation happens.
    fs::write(&output, "stale garbage\n").expect("unable to seed stale output");

    // Two writes in a row should not accumulate; final file should hold one run's worth.
    for _ in 0..2 {
        let res = json_to_rdf(
            &"tests/airplane.json".to_string(),
            &None,
            &Some(output.clone()),
        );
        assert!(res.is_ok());
    }

    let f = File::open(&output).expect("unable to open output file for result verification");
    let quads = RdfParser::from_format(RdfFormat::NTriples)
        .for_reader(f)
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse generated output file");

    assert_eq!(quads.len(), 23);
    let _ = fs::remove_file(&output);
}

#[test]
fn test_root_array() {
    let graph = json_to_rdf(&"tests/root_array.json".to_string(), &None, &None)
        .expect("conversion failed")
        .expect("expected graph");
    assert_eq!(graph.len(), 2);
}

#[test]
fn test_root_primitive_errors() {
    let result = json_to_rdf(&"tests/root_primitive.json".to_string(), &None, &None);
    assert!(matches!(
        result,
        Err(Json2RdfError::UnsupportedRootValue { kind: "number" })
    ));
}

#[test]
fn test_ndjson_stream_isolated() {
    let graph = json_to_rdf(&"tests/ndjson.json".to_string(), &None, &None)
        .expect("conversion failed")
        .expect("expected graph");
    assert_eq!(graph.len(), 2);
}

#[test]
fn test_hash_namespace_not_mangled() {
    let graph = json_to_rdf(
        &"tests/airplane.json".to_string(),
        &Some("http://example.com/ns#".to_string()),
        &None,
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
        &"tests/airplane.json".to_string(),
        &Some("http://example.com/ns/".to_string()),
        &None,
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
