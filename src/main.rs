use oxrdf::vocab::xsd;
use oxrdf::{
    graph, BlankNode, Graph, Literal, NamedNode, NamedNodeRef, NamedOrBlankNode, Term, TermRef,
    TripleRef,
};
use serde::de::value;
use serde_json::{Deserializer, Value};
use std::collections::{HashMap, VecDeque};
use std::fmt::format;
use std::fs::File;
use std::io::BufReader;
use uuid::Uuid;

fn main() {
    let file = File::open("src/airplane.json").unwrap();
    let reader = BufReader::new(file);
    let stream = Deserializer::from_reader(reader).into_iter::<Value>();

    let mut graph = Graph::default();

    let mut subject_stack: VecDeque<BlankNode> = VecDeque::new();
    let mut property: Option<String> = None;

    for value in stream {
        match value {
            Ok(Value::Object(obj)) => {
                let subject = BlankNode::default(); // Create a new blank node
                subject_stack.push_back(subject.clone());

                for (key, val) in obj {
                    property = Some(format!("http://decisym/data/json2rdf/#{}/", key));
                    process_value(&mut subject_stack, &property, val, &mut graph);
                }

                subject_stack.pop_back();
            }
            Ok(Value::Array(arr)) => {
                for val in arr {
                    process_value(&mut subject_stack, &property, val, &mut graph);
                }
            }
            Ok(other) => {
                process_value(&mut subject_stack, &property, other, &mut graph);
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
            }
        }
    }

    for triple in graph.iter() {
        println!(
            "Subject: {}, Predicate: {}, Object: {}",
            triple.subject, triple.predicate, triple.object
        );
    }
}

fn process_value(
    subject_stack: &mut VecDeque<BlankNode>,
    property: &Option<String>,
    value: Value,
    graph: &mut Graph,
) {
    if let Some(last_subject) = subject_stack.clone().back() {
        if let Some(prop) = property {
            match value {
                Value::Bool(b) => {
                    // println!("{},{},{}", subject_stack.back().unwrap(), prop, b);

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
                    // println!("{},{},{}", subject_stack.back().unwrap(), prop, literal);
                    graph.insert(TripleRef::new(
                        subject_stack.back().unwrap(),
                        NamedNodeRef::new(prop.as_str()).unwrap(),
                        &Literal::new_typed_literal(literal, xsd::INT),
                    ));
                }
                Value::String(s) => {
                    //println!("{},{},{}", subject_stack.back().unwrap(), prop, s);
                    graph.insert(TripleRef::new(
                        subject_stack.back().unwrap(),
                        NamedNodeRef::new(prop.as_str()).unwrap(),
                        &Literal::new_typed_literal(s, xsd::STRING),
                    ));
                }
                Value::Null => {
                    println!("Null value");
                }
                Value::Object(obj) => {
                    let subject = BlankNode::default();
                    subject_stack.push_back(subject);

                    graph.insert(TripleRef::new(
                        subject_stack.back().unwrap(),
                        NamedNodeRef::new(prop.as_str()).unwrap(),
                        subject_stack.back().unwrap(),
                    ));

                    for (key, val) in obj {
                        let nested_property =
                            Some(format!("http://decisym/data/json2rdf/#{}/", key));
                        process_value(subject_stack, &nested_property, val, graph);
                    }
                    subject_stack.pop_back();
                }
                Value::Array(arr) => {
                    for val in arr {
                        process_value(subject_stack, property, val, graph);
                    }
                }
            }
        }
    }
}
