use std::{
    path::{Path, PathBuf},
    vec,
};

use clap::Parser;
use lopdf::Document;
use mergedog::utils::docutils::{add_first_page_annotations, load_documents_from_path, merge};

const DEFAULT_FILE_NAME: &str = "merged.pdf";

/// Merge PDF's in specified directory.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to be searched
    #[arg(default_value_t = String::from("."))]
    inpath: String,

    /// The path of the output file
    #[arg(default_value_t = String::from(DEFAULT_FILE_NAME))]
    outpath: String,

    /// Annotate file names to corner of first slides
    #[arg(default_value_t = false, short, long)]
    anno: bool,

    /// Quiet
    #[arg(default_value_t = false, short, long)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();
    let inpath = PathBuf::from(args.inpath);
    let outpath = PathBuf::from(args.outpath);
    let verbose: bool = !args.quiet;
    if verbose {
        println!("Path:\n    {:?}", inpath)
    }
    let mut docs: Vec<(Document, String)> = load_documents_from_path(&inpath);
    if docs.len() == 0 {
        if verbose {
            println!("No PDFs found");
        }
        return;
    }
    docs.sort_by(|(_, a), (_, b)| a.cmp(b));
    if verbose {
        println!("Order:");
    }
    let mut file_names: Vec<String> = vec![];
    for (doc, name) in &docs {
        if verbose {
            println!("    Title: {}, Pages: {}", name, doc.get_pages().len());
        }
        file_names.push(name.to_string());
    }

    let merged_file_path = Path::new(&outpath);
    let mut merged_doc = merge(docs).unwrap();
    if args.anno {
        add_first_page_annotations(&mut merged_doc, file_names);
    }
    merged_doc.0.save(merged_file_path).unwrap();
    if verbose {
        println!("Merged:");
        println!(
            "    Path: {}, Pages: {}",
            outpath.to_str().unwrap(),
            merged_doc.0.get_pages().len()
        );
    }
}
