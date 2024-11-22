use json2rdf::json_to_rdf;

#[test]
fn test_graph_triple_count() {
    let triple_count_string = json_to_rdf(&"tests/airplane.json".to_string(), &None, &None);

    match triple_count_string {
        Ok(res) => {
            assert_eq!(res.unwrap().len(), 23)
        }
        Err(e) => eprintln!("Error writing: {}", e),
    }
}
