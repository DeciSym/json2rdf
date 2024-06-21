use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use rio_api::formatter::TriplesFormatter;
use rio_api::model::{NamedNode, Triple};
use rio_turtle::NTriplesFormatter;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

fn json_to_graph(value: &Value, graph: &mut Graph<String, String>, parent: Option<NodeIndex>) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let node = graph.add_node(key.clone());
                if let Some(parent_index) = parent {
                    graph.add_edge(parent_index, node, "".to_string());
                }
                json_to_graph(val, graph, Some(node));
            }
        }
        Value::Array(arr) => {
            for val in arr {
                json_to_graph(val, graph, parent);
            }
        }
        _ => {
            let node = graph.add_node(value.to_string());
            if let Some(parent_index) = parent {
                graph.add_edge(parent_index, node, "".to_string());
            }
        }
    }
}

fn graph_to_ttl(graph: &mut Graph<String, String>, file_path: &str) {
    let mut file = File::create(file_path);

    let mut formatter = NTriplesFormatter::new(Vec::default());

    for edge in graph.edge_references() {}
}

fn main() -> serde_json::Result<()> {
    let file_path = "/home/bharath/documents/github/json2rdf/src/airplane.json";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let json_value: Value = serde_json::from_reader(reader).unwrap();

    let mut graph = Graph::<String, String>::new();

    json_to_graph(&json_value, &mut graph, None);

    Ok(())
}
