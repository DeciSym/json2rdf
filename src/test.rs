use serde::de::value;
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, Read};

use rdf::node::Node;
use rdf::triple::Triple;
use rdf::uri::Uri;
use rio_api::model::*;
use rio_turtle::TurtleFormatter;
use std::io;

struct Graph {
    nodes: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: String, to: String) {
        self.nodes.entry(from).or_insert_with(Vec::new).push(to);
    }
}

fn json_to_graph(value: &Value, graph: &mut Graph, parent_key: Option<String>) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                let current_key = parent_key.clone().unwrap_or_else(|| "".to_string()) + "." + key;
                if let Some(parent) = &parent_key {
                    graph.add_edge(parent.clone(), current_key.clone());
                }
                json_to_graph(value, graph, Some(current_key));
            }
        }
        Value::Array(arr) => {
            for (index, value) in arr.iter().enumerate() {
                let current_key = parent_key.clone().unwrap_or_else(|| "".to_string())
                    + "["
                    + &index.to_string()
                    + "]";
                if let Some(parent) = &parent_key {
                    graph.add_edge(parent.clone(), current_key.clone());
                }
                json_to_graph(value, graph, Some(current_key));
            }
        }
        Value::String(string) => {
            println!("{}", string);
        }
        _ => {}
    }
}

fn main() -> serde_json::Result<()> {
    let file_path = "/home/bharath/documents/github/json2rdf/src/airplane.json";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let json_value: Value = serde_json::from_reader(reader).unwrap();

    let mut graph = Graph::new();
    json_to_graph(&json_value, &mut graph, None);

    for (node, edges) in &graph.nodes {
        println!("Node: {}", node);
        for edge in edges {
            println!("  -> {}", edge);
        }
    }

    Ok(())
}
