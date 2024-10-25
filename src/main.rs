use oxrdf::NamedOrBlankNode;
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::BufReader;
use struson::reader::*;

fn main() {
    // build using struson
    //-------------------------
    let mut json_reader = JsonStreamReader::new(
        r#"{
        "aircraft": {
            "id": "A12345",
            "model": "Boeing 747",
            "manufacturer": "Boeing",
            "capacity": {
                "seats": 416,
                "cargo_volume": 150.4
            }
        }
    }"#
        .as_bytes(),
    );

    let mut subject_stack: VecDeque<NamedOrBlankNode> = VecDeque::new();
    let mut array_properties: HashMap<NamedOrBlankNode, NamedOrBlankNode> = HashMap::new();

    json_reader.begin_object().unwrap();
    json_reader.next_name().unwrap();

    // println!("hi");

    // json_reader.begin_array().unwrap();
    // assert_eq!(json_reader.next_number::<u32>().unwrap().unwrap(), 1);
    // assert_eq!(json_reader.next_bool().unwrap(), true);
    // json_reader.end_array().unwrap();

    json_reader.begin_object().unwrap();

    while json_reader.has_next().unwrap() {
        match json_reader.peek().unwrap() {
            ValueType::Array => match token {
                if token 
            },
            ValueType::Object => todo!(),
            ValueType::String => println!("A string: {}", json_reader.next_string().unwrap()),
            ValueType::Boolean => println!("a bool: {}", json_reader.next_bool().unwrap()),
            ValueType::Number => println!(
                "A string: {}",
                json_reader.next_number::<u32>().unwrap().unwrap()
            ),

            _ => panic!("Unexpected type"),
        }
    }

    json_reader.end_object().unwrap();
    // Ensures that there is no trailing data
    json_reader.consume_trailing_whitespace().unwrap();
    //-------------------------

    //built using serde_json
    //-------------------------
    // let file = File::open("src/airplane.json").unwrap();
    // let reader = BufReader::new(file);

    // // Parse the JSON file into a serde_json::Value
    // let json_data: Value = serde_json::from_reader(reader).unwrap();
    // parse_json(&json_data);
    //-------------------------
}

fn parse_json(value: &Value) {
    match value {
        Value::Null => println!("null"),
        Value::Bool(b) => println!("Boolean: {}", b),
        Value::Number(n) => println!("Number: {}", n),
        Value::String(s) => println!("String: {}", s),
        Value::Array(arr) => {
            println!("Array:");
            for (index, item) in arr.iter().enumerate() {
                print!("Index {}: ", index);
                parse_json(item);
            }
        }
        Value::Object(obj) => {
            println!("Object:");
            for (key, val) in obj.iter() {
                print!("Key: {}, Value: ", key);
                parse_json(val);
            }
        }
    }
}


// psuedocode
// subject array
// properties array 

// property = null

// While Next val exists

//     if start of array 
        
//     if end of array

//     if start of object
//         create blank node
//         if property exists and subject isnt empty create triples with subject array.last(),property,subject)
//         add blank node to subject array to track
//     if end of object
//         .pop() on subject array
//         WHAT DOES THIS MEAN( if (!subjectStack.isEmpty() && arrayProperties.containsKey(subjectStack.getLast())) property = arrayProperties.get(subjectStack.getLast());)
//     if string
//         create triple with last element of blank node array, property , string val
//     if number
//         create triple with last element of blank node array, property , number val
//     if bool
//         create triple with last element of blank node array, property , bool val
//     if key_name
//         property = key_name
