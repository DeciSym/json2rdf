use oxrdf::{LiteralRef, NamedNodeRef, TermRef, TripleRef};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
// use rio_api::formatter::TriplesFormatter;
// use rio_api::model::{NamedNode, Triple};
// use rio_turtle::NTriplesFormatter;
use serde_json::Value;
use std::fmt::format;
use std::fs::{read, File};
use std::io::{BufReader, Write};

fn json_to_graph(value: &Value, graph: &mut Graph<String, String>, parent: Option<NodeIndex>) {
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                let node = graph.add_node(key.clone());
                if let Some(parent_index) = parent {
                    graph.add_edge(parent_index, node, "".to_string());
                }
                // let node = graph.add_node("".to_string());
                // if let Some(parent_index) = parent {
                //     graph.add_edge(parent_index, node, key.to_string());
                // }
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
                // let node = graph.add_node("".to_string());
                // if let Some(parent_index) = parent {
                //     graph.add_edge(parent_index, node, value.to_string());
            }
        }
    }
}

fn graph_to_ttl(graph: &mut Graph<String, String>, file_path: &str) {
    let mut file = File::create(file_path);

    let mut triples_graph = oxrdf::Graph::default();

    for edge in graph.edge_references() {
        let val = format!("http://www.decisym.ai/data#{}", &graph[edge.source()]);

        let subject_triple = NamedNodeRef::new(val.as_str()).unwrap();

        let predicate_triple = NamedNodeRef::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");

        let object_triples = TermRef::Literal(LiteralRef::new_simple_literal(
            graph[edge.target()].as_str(),
        ));

        let rdf_triple = TripleRef::new(subject_triple, predicate_triple.unwrap(), object_triples);

        triples_graph.insert(rdf_triple);
    }

    writeln!(file.unwrap(), "{}", triples_graph.to_string()).unwrap();
}

fn main() -> serde_json::Result<()> {
    let file_path = "/home/bharath/documents/github/json2rdf/src/airplane.json";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let json_value: Value = serde_json::from_reader(reader).unwrap();

    let mut graph = Graph::<String, String>::new();

    json_to_graph(&json_value, &mut graph, None);

    // for edge in graph.edge_references() {
    //     println!("{:?}", graph[edge.source()]);
    //     println!("{:?}", graph[edge.target()]);

    //     println!("------------")
    //     //println!("{:?}", edge.id());
    // }

    graph_to_ttl(&mut graph, "/home/bharath/test.ttl");

    Ok(())
}
