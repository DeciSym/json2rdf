// Copyright (c) 2024-2025, DeciSym, LLC
// Licensed under either of:
// - Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// - BSD 3-Clause License (https://opensource.org/licenses/BSD-3-Clause)
// at your option.

use json2rdf::json_to_rdf;
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
    let _ = fs::remove_file(output.clone());

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
