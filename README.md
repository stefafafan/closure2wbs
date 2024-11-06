# closure2wbs

closure2wbs is a cli tool for converting a list of closure tables to a WBS structure for PlantUML.
Currently, the tool assumes a JSON file input, and outputs to file.

## Usage
Try `--help`

```sh
Usage: closure2wbs [OPTIONS]

Options:
  -f, --filename <FILENAME>  [default: closures.json]
  -o, --output <OUTPUT>      [default: closures_wbs.puml]
  -h, --help                 Print help
  -V, --version              Print version
  ```

  ### Example

Prepare a json file like following:

  ```json
[
	{
		"ancestor": "A",
		"descendant": "A"
	},
	{
		"ancestor": "A",
		"descendant": "B"
	},
	{
		"ancestor": "B",
		"descendant": "B"
	},
	{
		"ancestor": "B",
		"descendant": "C"
	},
	{
		"ancestor": "C",
		"descendant": "C"
	}
]
  ```

Run the cli tool.

```sh
cargo run -- -f closures.json -o out.puml
```

`out.puml` contents will be like as the following.

```pml
@startwbs
* A
** B
*** C
@endwbs
```
