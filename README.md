# ntbk

`ntbk` is a terminal notebook manager written in rust.

## Installation

Clone this repository and `cd` into it.

```bash
git clone git@github.com:maaslalani/ntbk.git && cd ntbk
```

Edit the `config.rs` file with your configuration.
```bash
$EDITOR src/config.rs
```

Install `ntbk` with cargo.
```bash
cargo install --path .
```

## Usage

```
ntbk find   <pattern>  - Find notes by path
ntbk grep   <pattern>  - Search through notes content
ntbk help              - Print this usage information
ntbk list              - List all notes
ntbk new    <name>     - Create a new note with <name>
ntbk open   <name>     - Open a note
ntbk remove <name>     - Remove a note
ntbk show   <name>     - Show a note's content
```

## Contributing
Pull requests are welcome.

## License
[MIT](https://choosealicense.com/licenses/mit/)
