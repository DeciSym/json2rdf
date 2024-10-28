use oxrdf::{Graph, NamedOrBlankNode};
use serde::de::value;
use serde_json::{Deserializer, Value};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("src/airplane.json").unwrap();
    let reader = BufReader::new(file);
    let stream = Deserializer::from_reader(reader).into_iter::<Value>();

    let mut subject_stack: VecDeque<String> = VecDeque::new();
    let mut array_properties: HashMap<String, String> = HashMap::new();
    let mut property: Option<String> = None;

    for value in stream {
        match value {
            Ok(Value::Object(obj)) => {
                let subject = format!("_:b{}", subject_stack.len()); // Create a new blank node
                println!("Created new subject: {}", subject);
                subject_stack.push_back(subject.clone());

                if let Some(last_subject) = subject_stack.back() {
                    println!("The last subject in the stack is: {}", last_subject);
                    if let Some(prop) = &property {
                        println!("Adding property for parent -> child relationship: {}", prop);
                    }
                }

                for (key, val) in obj {
                    property = Some(format!("#{}", key));
                    println!("Processing key: {}", key);
                    process_value(&mut subject_stack, &property, val);
                }

                // End of object; remove the subject from stack
                subject_stack.pop_back();
            }
            Ok(Value::Array(arr)) => {
                if let Some(last_subject) = subject_stack.back() {
                    if let Some(prop) = &property {
                        array_properties.insert(last_subject.clone(), prop.clone());
                    }
                }
                for val in arr {
                    process_value(&mut subject_stack, &property, val);
                }
            }
            Ok(other) => {
                process_value(&mut subject_stack, &property, other);
            }
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
            }
        }
    }
}

fn process_value(subject_stack: &mut VecDeque<String>, property: &Option<String>, value: Value) {
    if let Some(last_subject) = subject_stack.back() {
        println!("The last subject in the stack is: {}", last_subject);
        if let Some(prop) = property {
            println!("Processing property: {}", prop);
            match value {
                Value::Bool(b) => {
                    println!("Boolean value: {}", b);
                }
                Value::Number(num) => {
                    let literal = if let Some(int) = num.as_i64() {
                        int.to_string()
                    } else if let Some(float) = num.as_f64() {
                        float.to_string()
                    } else {
                        return;
                    };
                    println!("Number value: {}", literal);
                }
                Value::String(s) => {
                    println!("String value: {}", s);
                }
                Value::Null => {
                    println!("Null value");
                }
                Value::Object(obj) => {
                    let subject = format!("_:b{}", subject_stack.len());
                    println!("Created nested subject: {}", subject);
                    subject_stack.push_back(subject);
                    for (key, val) in obj {
                        let nested_property = Some(format!("#{}", key));
                        process_value(subject_stack, &nested_property, val);
                    }
                    subject_stack.pop_back();
                }
                Value::Array(arr) => {
                    for val in arr {
                        process_value(subject_stack, property, val);
                    }
                }
            }
        }
    }
}
