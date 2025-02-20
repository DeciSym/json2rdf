# json2rdf

This Rust-based tool converts JSON data into RDF format, utilizing the `oxrdf` crate for RDF graph handling and `serde_json` for efficient JSON parsing. It supports lightweight, memory-efficient processing by reading JSON data values sequentially, which makes it suitable for large datasets.

## Using the json2rdf CLI

This library includes a CLI utility for parsing JSON and generating N-Triple RDF using the `convert` subcommand. The binary can be built using `cargo build`.

```bash
$ json2rdf convert --help
Convert JSON to RDF format.

The `convert` command parses a JSON file, converts it to RDF triples using `serde_json` for parsing and `oxrdf` to construct the graph, and saves the output.

Usage: json2rdf convert [OPTIONS] --json-files <JSON_FILES>

Options:
  -n, --namespace <NAMESPACE>
          Namespace for RDF graph generation.

          A custom namespace to prefix RDF resources created from JSON keys and values.

  -j, --json-files <JSON_FILES>
          Path to input JSON file(s).

          Provide the path to one or more JSON files that will be parsed and converted.

  -o, --output-file <OUTPUT_FILE>
          Path to output file.

          Optional: Specify the path to save the generated RDF data.

  -h, --help
          Print help (see a summary with '-h')
```

## Using the convert library

The conversion functionality can also be called directly in Rust. The library supports writing results to a file or building an in-memory `oxrdf::Graph`.

```rust
use json2rdf::json_to_rdf;

// capture conversion results to file
let results = json_to_rdf(&"tests/airplane.json".to_string(), &Some("http://example.com/ns#".to_string()), &Some("output.nt".to_string()));

// capture conversion results to an oxrdf::Graph
let results = json_to_rdf(&"tests/airplane.json".to_string(), &Some("http://example.com/ns#".to_string()), &None);
```

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- BSD 3-Clause License ([LICENSE-BSD-3](LICENSE-BSD-3) or
  https://opensource.org/licenses/BSD-3-Clause)

at your option.