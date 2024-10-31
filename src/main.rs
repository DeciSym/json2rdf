use clap::{Parser, Subcommand};
mod lib;
/// JSON2RDF Converter
/// This tool converts JSON data into RDF format.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// The JSON2RDF CLI
    Convert {
        /// namespace to be used for graph generation
        #[arg(short, long)]
        namespace: Option<String>,
        /// path to JSON file(s)
        #[arg(short, long)]
        json_files: String,
        /// write triples from oxrdf graph to a file
        #[arg(short, long)]
        output_file: Option<String>,
    },
}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Convert {
            namespace,
            json_files,
            output_file,
        }) => {
            if Some(output_file).is_some() {
                lib::json_to_rdf(json_files, namespace, output_file);
            } else {
                lib::json_to_rdf(json_files, namespace, &None);
            }
        }
        None => {}
    }
}
