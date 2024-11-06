use std::fs;

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

fn json_to_plantuml_wbs(json: Vec<serde_json::Value>) -> String {
    // TODO: When using wbs, the contents should be output in a way using *.
    // Like this:
    // @startwbs
    // * A
    // ** B
    let mut plantuml = String::from("@startwbs\n");

    let root_node = retrieve_root_node(json);
    println!("Root node: {}", root_node);
    plantuml.push_str(&format!("* {}\n", root_node));

    // for record in json_to_parse {
    //     let closure_record: ClosureRecord =
    //         serde_json::from_value(record).expect("Unable to deserialize");
    //     plantuml.push_str(&format!(
    //         "{} -> {}\n",
    //         closure_record.ancestor, closure_record.descendant
    //     ));
    // }

    plantuml.push_str("@endwbs\n");
    plantuml
}

fn main() {
    let json = fs::read_to_string("closures.json").expect("Unable to read file");
    let deserialized: Vec<serde_json::Value> =
        serde_json::from_str(&json).expect("Unable to deserialize");
    let plantuml = json_to_plantuml_wbs(deserialized);

    fs::write("closures_wbs.puml", plantuml).expect("Unable to write file");
}
