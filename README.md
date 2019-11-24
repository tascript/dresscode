# dresscode
[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/dresscode)](https://crates.io/crates/dresscode)

dresscode dress up stdin every keywords.

## Install
```
cargo install dresscode 
```

## Usage
```
USAGE:
    dresscode [KEYWORD]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <KEYWORD>...    Keyword
```

**example**

```
 $ tail -f README.md | dresscode Usage ARGS Keyword License
 ```

## License
MIT
