use oxrdf::vocab::xsd;
use oxrdf::{BlankNode, Graph, Literal, NamedNodeRef, TripleRef};

use serde_json::{Deserializer, Value};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, Write};

pub fn json_to_rdf(file_path: &String, namespace: &Option<String>, output_file: &Option<String>) {
    let rdf_namespace: String = if namespace.is_some() {
        namespace.clone().unwrap()
    } else {
        "https://decisym/data".to_owned()
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

    let mut buffer: Vec<u8> = Vec::new();
    writeln!(buffer, "{}", graph.to_string()).unwrap();

    let file = File::create("foo.txt");
    writeln!(file.unwrap(), "{}", String::from_utf8_lossy(&buffer)).unwrap();
}

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
