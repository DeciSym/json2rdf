use json2rdf::json_to_rdf;

#[test]
fn test_graph_triple_count() {
    let triple_count_string = json_to_rdf(&"tests/airplane.json".to_string(), &None, &None);

    match triple_count_string {
        Ok(res) => match res {
            json2rdf::GraphOrMessage::Graph(graph) => {
                // Handle the case where the function returns a Graph
                println!("Graph created with {} triples", graph.len());
                assert_eq!(graph.len(), 23)
            }
            json2rdf::GraphOrMessage::Message(message) => {
                println!("{}", message);
            }
        },
        Err(e) => eprintln!("Error writing: {}", e),
    }
}
