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

fn main() {
    // -> serde_json::Result<()>
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

    graph_to_ttl(
        &mut graph,
        "/home/bharath/documents/github/json2rdf/outputfile.ttl",
    );

    // Ok(())
}

// // vibefpk9348o4cawzkgiuotfrkgcvxgqphert24v8d9t9

//     // -------------------------------------------------------
//     // new json2rdf code
//     use struson::reader::*;

//     let json = r#"{"a": ["1", "true"]}"#;
//     let mut json_reader = JsonStreamReader::new(json.as_bytes());

//     json_reader.begin_object().unwrap();

//     // let mut subject_stack: Vec<Box<dyn Any>> = vec![];
//     // let mut array_properties: HashMap<String, Box<dyn Any>> = HashMap::new();

//     while json_reader.has_next().expect("Filed") {
//         // match json_reader.peek().unwrap() {
//         //     //ValueType::Array => todo!(),
//         //     ValueType::Boolean => println!("A boolean: {}", json_reader.next_bool().unwrap()),
//         //     ValueType::Object => todo!(),
//         //     ValueType::String => println!("A string: {}", json_reader.next_str().unwrap()),
//         //     ValueType::Number => todo!(),
//         //     ValueType::Null => todo!(),
//         //     _ => panic!("Unexpected type"),
//         json_reader.skip_value();
//         println!("hi");
//         //}
//     }
//     json_reader.end_object().unwrap();
//
//use std::{any::Any, collections::HashMap};
