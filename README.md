# query-string

Parse and stringify URL query strings in a fantastic way.

## Features

- **URL Encoding**: Encodes characters that are not part of the unreserved set (`A-Z`, `a-z`, `0-9`, `-`, `_`, `.`, `~`) and properly handles multi-byte UTF-8 characters.
- **URL Decoding**: Decodes percent-encoded characters back to their original form.
- **Query String Parsing**: Parses a query string into a `HashMap` of key-value pairs.
- **Query String Stringifying**: Converts a `HashMap` of key-value pairs back into a query string.

## Installation

### Install with Cargo CLI:

```bash
cargo add query_string
```

or

### Add this to your `Cargo.toml`:

```toml
[dependencies]
query_string = "0.1.0"
```

## Usage

### Encoding

The encode function encodes any character that is not part of the unreserved set and handles multi-byte UTF-8 characters.

```rs
use your_crate_name::encode;

fn main() {
    let input = "Hello World! こんにちは";
    let encoded = encode(input);
    println!("{}", encoded); // Output: Hello%20World%21%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF
}
```

### Decoding

The decode function decodes a percent-encoded string.

```rs
use your_crate_name::decode;

fn main() -> std::io::Result<()> {
    let encoded = "Hello%20World%21%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF";
    let decoded = decode(encoded)?;
    println!("{}", decoded); // Output: Hello World! こんにちは
    Ok(())
}
```

### Query String Parsing

The parse function parses a query string into a `HashMap<String, String>`.

```rs
use your_crate_name::parse;

fn main() {
   use query_string::parse;

fn main() {
    let query = "?foo=bar&foo=baz&name=John&age=30";
    let parsed = parse(query);

    for (key, values) in &parsed {
        println!("{}: {:?}", key, values);
    }
    // Output:
    // foo: ["bar", "baz"]
    // name: ["John"]
    // age: ["30"]
}
}
```

### Query String Stringifying

The stringify function converts a `HashMap<String, String>` into a query string.

```rs
use std::collections::HashMap;
use your_crate_name::stringify;

fn main() {
    let mut params = HashMap::new();
    params.insert("name".to_string(), "John".to_string());
    params.insert("age".to_string(), "30".to_string());
    params.insert("city".to_string(), "New York".to_string());

    let query_string = stringify(&params);
    println!("{}", query_string); // Output: name=John&age=30&city=New York
}
```

## Functions

### `encode(input: &str) -> String`

Encodes the input string using percent-encoding. Reserved characters and non-ASCII characters are encoded as %XX, where XX is the hexadecimal representation of the byte.

### `decode(input: &str) -> io::Result<String>`

Decodes a percent-encoded string. Returns an error if the input contains invalid percent-encoded sequences.

### `parse(query: &str) -> HashMap<String, Vec<String>>`

Parses a URL query string into a `HashMap<String, Vec<String>>`. Handles removing unnecessary characters like ?, #, and ! from the query string before parsing.

### `stringify(params: &HashMap<String, String>) -> String`

Converts a HashMap<String, String> into a query string format, where key-value pairs are joined by = and separated by &.

## Contribution

All contribution are welcome. Please create issue or make PR for bugs, issues or new features. Please feel free to reach out for detail information of the project.

## License

This project is licensed under the MIT License. See the [LICENSE](/LICENSE) file for details.
