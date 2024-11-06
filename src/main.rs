use std::fs;

use clap::Parser;

const DEFAULT_FILENAME: &str = "closures.json";
const DEFAULT_OUTPUT_FILENAME: &str = "closures_wbs.puml";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = DEFAULT_FILENAME.to_string())]
    filename: String,

    #[arg(short, long, default_value_t = DEFAULT_OUTPUT_FILENAME.to_string())]
    output: String,
}

#[derive(serde::Deserialize)]
struct ClosureRecord {
    ancestor: String,
    descendant: String,
}

fn retrieve_root_node(json: Vec<serde_json::Value>) -> String {
    let mut ancestors: Vec<String> = Vec::new();
    let mut descendants: Vec<String> = Vec::new();

    for record in json {
        let closure_record: ClosureRecord =
            serde_json::from_value(record).expect("Unable to deserialize");
        ancestors.push(closure_record.ancestor);
        descendants.push(closure_record.descendant);
    }

    let mut descendants_per_ancestor: std::collections::HashMap<String, i32> =
        std::collections::HashMap::new();
    for descendant in descendants {
        if ancestors.contains(&descendant) {
            let val = descendants_per_ancestor.get(&descendant);
            match val {
                Some(v) => {
                    descendants_per_ancestor.insert(descendant, v + 1);
                }
                None => {
                    descendants_per_ancestor.insert(descendant, 1);
                }
            }
        }
    }

    for (key, value) in descendants_per_ancestor {
        if value == 1 {
            return key;
        }
    }

    "".to_string()
}

fn closure_children_to_wbs_string(
    json: Vec<serde_json::Value>,
    current: String,
    level: i32,
) -> String {
    let mut tempstr;
    tempstr = format!("{} {}\n", "*".repeat(level as usize), current);

    let mut children: Vec<String> = Vec::new();

    for record in json.clone() {
        let closure_record: ClosureRecord =
            serde_json::from_value(record).expect("Unable to deserialize");
        if closure_record.ancestor == current && closure_record.descendant != current {
            children.push(closure_record.descendant);
        }
    }

    for child in children {
        tempstr.push_str(&closure_children_to_wbs_string(
            json.clone(),
            child,
            level + 1,
        ));
    }

    tempstr.to_string()
}

fn json_to_plantuml_wbs(json: Vec<serde_json::Value>) -> String {
    let mut plantuml = String::from("@startwbs\n");
    let root_node = retrieve_root_node(json.clone());
    plantuml.push_str(&closure_children_to_wbs_string(json.clone(), root_node, 1));
    plantuml.push_str("@endwbs\n");
    plantuml
}

fn main() {
    let args = Args::parse();
    let json = fs::read_to_string(args.filename).expect("Unable to read file");
    let deserialized: Vec<serde_json::Value> =
        serde_json::from_str(&json).expect("Unable to deserialize");
    let plantuml = json_to_plantuml_wbs(deserialized);

    fs::write(args.output, plantuml).expect("Unable to write file");
}
