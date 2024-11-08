use std::fs;

use clap::Parser;

// XXX: Instead of using files, support reading from stdin and writing to stdout
const DEFAULT_FORMAT: &str = "plantuml";
const DEFAULT_FILENAME: &str = "input.json";
const DEFAULT_OUTPUT_FILENAME: &str = "output.txt";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = DEFAULT_FORMAT.to_string())]
    format: String,

    #[arg(short, long, default_value_t = DEFAULT_FILENAME.to_string())]
    input: String,

    #[arg(short, long, default_value_t = DEFAULT_OUTPUT_FILENAME.to_string())]
    output: String,
}

// XXX: Currently assumes that the input JSON is a list of objects with `ancestor` and `descendant` keys.
#[derive(serde::Deserialize)]
struct ClosureRecord {
    ancestor: String,
    descendant: String,
    depth: i32,
}

// Retrieves the root node of the tree.
// The root node is the node that is an ancestor of all other nodes, but has no ancestor itself.
// This is done by counting the number of times a node appears as a descendant.
// If a node appears as a descendant only once, it is the root node.
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

    // The program assumes that the root node is always present, if it is not that means it is some cyclic graph, which is not assumed.
    panic!("Root node not found")
}

// Recursively generates the WBS string from the JSON. The level is used to determine the number of asterisks to prepend to the node (used for printing the PlantUML WBS).
fn closure_children_to_plantuml_string(
    json: Vec<serde_json::Value>,
    current: String,
    level: i32,
) -> String {
    let mut wbs_string;
    wbs_string = format!("{} {}\n", "*".repeat(level as usize), current);

    let mut children: Vec<String> = Vec::new();

    // Find all direct children of the current node.
    for record in json.clone() {
        let closure_record: ClosureRecord =
            serde_json::from_value(record).expect("Unable to deserialize");
        if closure_record.ancestor == current
            && closure_record.descendant != current
            && closure_record.depth == 1
        {
            children.push(closure_record.descendant);
        }
    }

    // Recursively call the function for each direct child.
    for child in children {
        wbs_string.push_str(&closure_children_to_plantuml_string(
            json.clone(),
            child,
            level + 1,
        ));
    }

    wbs_string.to_string()
}

fn closure_children_to_mermaid_string(json: Vec<serde_json::Value>, current: String) -> String {
    let mut mermaid_string: String = String::new();

    let mut children: Vec<String> = Vec::new();

    // Find all direct children of the current node.
    for record in json.clone() {
        let closure_record: ClosureRecord =
            serde_json::from_value(record).expect("Unable to deserialize");
        if closure_record.ancestor == current
            && closure_record.descendant != current
            && closure_record.depth == 1
        {
            children.push(closure_record.descendant);
        }
    }

    // Recursively call the function for each direct child.
    for child in children {
        mermaid_string.push_str(&format!("{} --> {}\n", current, child));
        mermaid_string.push_str(&closure_children_to_mermaid_string(json.clone(), child));
    }

    mermaid_string.to_string()
}

// Converts the JSON to a PlantUML WBS string.
fn json_to_plantuml_wbs(json: Vec<serde_json::Value>) -> String {
    let mut plantuml = String::from("@startwbs\n");

    // Starting from the root node, recursively generate the WBS string.
    let root_node = retrieve_root_node(json.clone());
    plantuml.push_str(&closure_children_to_plantuml_string(
        json.clone(),
        root_node,
        1,
    ));

    plantuml.push_str("@endwbs\n");
    plantuml
}

fn json_to_mermaid_wbs(json: Vec<serde_json::Value>) -> String {
    let mut mermaid = String::from("flowchart TD\n");

    // Starting from the root node, recursively generate the WBS string.
    let root_node = retrieve_root_node(json.clone());
    mermaid.push_str(&closure_children_to_mermaid_string(json.clone(), root_node));

    mermaid
}

fn main() {
    let args = Args::parse();
    let json = fs::read_to_string(args.input).expect("Unable to read file");
    let deserialized: Vec<serde_json::Value> =
        serde_json::from_str(&json).expect("Unable to deserialize");

    let contents = match args.format.as_str() {
        "plantuml" => json_to_plantuml_wbs(deserialized.clone()),
        "mermaid" => json_to_mermaid_wbs(deserialized.clone()),
        _ => panic!("Invalid format"),
    };

    fs::write(args.output, contents).expect("Unable to write file");
}
