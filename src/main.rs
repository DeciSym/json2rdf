use struson::reader::*;

fn main() {
    // let json = r#"{"a": true}"#;
    // let mut json_reader = JsonStreamReader::new(json.as_bytes());

    let mut json_reader = JsonStreamReader::new(r#"{"a": 1, "b": 2}"#.as_bytes());
    json_reader.begin_object().unwrap();

    while json_reader.has_next().unwrap() {
        match json_reader.peek().unwrap() {
            ValueType::Boolean => println!("A boolean: {}", json_reader.next_bool().unwrap()),
            ValueType::String => println!("A string: {}", json_reader.next_str().unwrap()),
            _ => panic!("Unexpected type"),
        }

        // json_reader.skip_name().unwrap();
        // json_reader.skip_value().unwrap();
        // println!("SKIPPED");
    }

    json_reader.end_object().unwrap();
}
