mod identifier;
mod svg;
mod yml;

use std::fs;

use svg::document::Document;
use svg::element::Element;
use svg::point::Point;
use svg::rectangle::Rectangle;
use svg::text::Text;

extern crate linked_hash_map;
extern crate yaml_rust;
use linked_hash_map::LinkedHashMap;
use yaml_rust::{ScanError, Yaml, YamlLoader};

#[derive(Debug)]
enum EmlElement {
    FormElement,
    JobElement,
    CommandElement,
    EventElement,
    ViewElement,
}

// fn ingest_node_type(key: &Yaml) {
//     match key {
//         Yaml::String("Form") => println! {"Form"},
//         _ => println!("other"),
//     }
// }
//

fn ingest_eml_node_hash(hsh: &LinkedHashMap<Yaml, Yaml>) {
    let keycount = hsh.keys().len();
    match keycount {
        1 => {
            println!("{:?}", hsh.keys()[0])
        }
        _ => panic!("expected 1 key in eml node"),
    }
}

fn ingest_eml_node(item: &Yaml) {
    match item {
        Yaml::Hash(hsh) => ingest_eml_node_hash(hsh),
        _ => panic!("node should be a Yaml::Hash"),
    }
}

fn ingest_docs(docs: Vec<Yaml>) {
    match &docs[..] {
        [doc] => match doc {
            Yaml::Array(arr) => {
                // replace with map()::collect() -> Vec
                for item in arr.iter() {
                    ingest_eml_node(item)
                }
            }
            _ => panic!("top level should be Yaml::Array"),
        },
        _ => panic!("too many docs"),
    }
}

fn ingest(ymlstr: String) -> Result<String, String> {
    match YamlLoader::load_from_str(&ymlstr) {
        Ok(docs) => {
            ingest_docs(docs);
            Ok("ok".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    match fs::read_to_string("model.yaml") {
        Ok(rawdata) => println!("{:?}", ingest(rawdata)),
        Err(e) => panic!("{:?}", e),
    }
}
