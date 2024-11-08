# closure2wbs

closure2wbs is a cli tool for converting a list of closure tables to a WBS structure for PlantUML or Mermaid.
Currently, the tool assumes a JSON file input, and outputs to file.

> [!WARNING]
> This cli tool is not thoroughly tested! 

## Installation
Assuming you have `cargo` setup:

```sh
cargo install --git https://github.com/stefafafan/closure2wbs
```

## Usage
Try `--help`

```sh
$ closurewbs --help

A cli tool to convert closure tables to a WBS representation.

Usage: closure2wbs [OPTIONS]

Options:
  -f, --format <FORMAT>  [default: plantuml]
  -i, --input <INPUT>    [default: input.json]
  -o, --output <OUTPUT>  [default: output.txt]
  -h, --help             Print help
  -V, --version          Print version
  ```

### Example

Prepare a json file like following:

```json
[
	{
		"ancestor": "A",
		"descendant": "A",
		"depth": 0
	},
	{
		"ancestor": "A",
		"descendant": "B",
		"depth": 1
	},
	{
		"ancestor": "A",
		"descendant": "C",
		"depth": 1
	},
	{
		"ancestor": "A",
		"descendant": "D",
		"depth": 2
	},
	{
		"ancestor": "A",
		"descendant": "E",
		"depth": 2
	},
	{
		"ancestor": "B",
		"descendant": "B",
		"depth": 0
	},
	{
		"ancestor": "B",
		"descendant": "D",
		"depth": 1
	},
	{
		"ancestor": "B",
		"descendant": "E",
		"depth": 1
	},
	{
		"ancestor": "C",
		"descendant": "C",
		"depth": 0
	}
]
```

Run the cli tool.

```sh
closure2wbs --input input.json --output out.puml --format plantuml
```

`out.puml` contents will be like as the following.

```pml
@startwbs
* A
** B
*** D
*** E
** C
@endwbs
```

Mermaid output is supported as well:

```sh
closure2wbs --input input.json --output out.mmd --format mermaid
```

```mermaid
flowchart TD
A --> B
B --> D
B --> E
A --> C
```
