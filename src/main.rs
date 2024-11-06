use std::fs;

#[derive(serde::Deserialize)]
struct ClosureRecord {
    ancestor: String,
    descendant: String,
}

fn json_to_plantuml_wbs(json: Vec<serde_json::Value>) -> String {
    // TODO: When using wbs, the contents should be output in a way using *.
    // Like this:
    // @startwbs
    // * A
    // ** B
    let mut plantuml = String::from("@startwbs\n");
    for record in json {
        let closure_record: ClosureRecord =
            serde_json::from_value(record).expect("Unable to deserialize");
        plantuml.push_str(&format!(
            "{} -> {}\n",
            closure_record.ancestor, closure_record.descendant
        ));
    }
    plantuml.push_str("@endwbs");
    plantuml
}

fn main() {
    let json = fs::read_to_string("closures.json").expect("Unable to read file");
    let deserialized: Vec<serde_json::Value> =
        serde_json::from_str(&json).expect("Unable to deserialize");
    let plantuml = json_to_plantuml_wbs(deserialized);

    fs::write("closures_wbs.puml", plantuml).expect("Unable to write file");
}
