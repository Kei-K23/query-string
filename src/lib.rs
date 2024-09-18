mod helper;

use std::{collections::HashMap, io};

use helper::remove_char;

/// Encode function encode any character that is not part of the unreserved set (A-Z, a-z, 0-9, -, _, ., ~) and properly handle multi-byte UTF-8 characters.
pub fn encode(input: &str) -> String {
    let mut encoded = String::new();

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.' || ch == '~' {
            encoded.push(ch);
        } else {
            // Handle reserved character
            let utf8_bytes = ch.to_string().into_bytes();
            for byte in utf8_bytes {
                encoded.push('%');
                encoded.push_str(&format!("{:02x}", byte)); // format as a two-digit uppercase hexadecimal number
            }
        }
    }

    encoded
}

pub fn decode(input: &str) -> io::Result<String> {
    let mut decoded = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            // percent encoded character
            // Ensure we have exactly 2 hex digits after '%'
            let hex1 = chars.next().unwrap();
            let hex2 = chars.next().unwrap();

            // Convert the two hex digits into a byte
            let hex = format!("{}{}", hex1, hex2);
            let byte = u8::from_str_radix(&hex, 16)
                .map_err(|_| format!("Invalid encoding: invalid hex digits '{}'", hex))
                .unwrap();

            decoded.push(byte as char);
        } else {
            // Non-percent encoded character, add directly
            decoded.push(ch);
        }
    }

    Ok(decoded)
}

/// Parse query string into key value pair
pub fn parse(query: &str) -> HashMap<String, Vec<String>> {
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();

    // Remove unnecessary part of query '?#!'
    let query_str = remove_char(query, "?#!");

    for query in query_str.split("&") {
        let mut pairs = query.split("="); // Split the key-value pair

        let key = pairs.next().unwrap_or_default();
        let value = pairs.next().unwrap_or_default();

        // Insert or append the value to the existing key
        hashmap
            .entry(key.to_string())
            .or_insert_with(Vec::new)
            .push(value.to_string());
    }

    hashmap
}

pub fn stringify(params: &HashMap<String, String>) -> String {
    params
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let original = "Hello World!";
        let encoded = encode(&original);

        assert_eq!(encoded, "Hello%20World%21");
    }

    #[test]
    fn test_decode() {
        let encoded = "Hello%20World%21";
        let decoded = decode(&encoded).unwrap();

        assert_eq!(decoded, "Hello World!");
    }

    #[test]
    fn test_parse() {
        let query = "foo=bar&baz=qux";
        let result = parse(query);

        let result1 = result.get("foo").unwrap();
        let result2 = result.get("baz").unwrap();

        assert_eq!(result1[0], "bar");
        assert_eq!(result2[0], "qux");
    }

    #[test]
    fn test_parse_multiple_value() {
        let query = "foo=bar&foo=qux";
        let result = parse(query);

        let result1 = result.get("foo").unwrap();

        assert_eq!(result1.len(), 2);
        assert_eq!(result1[0], "bar");
        assert_eq!(result1[1], "qux");
    }

    #[test]
    fn test_parse_with_leading_character() {
        let query = "?foo=bar&baz=qux";
        let result = parse(query);

        let result1 = result.get("foo").unwrap();
        let result2 = result.get("baz").unwrap();

        assert_eq!(result1[0], "bar");
        assert_eq!(result2[0], "qux");
    }

    #[test]
    fn test_stringify() {
        let mut params = HashMap::new();
        params.insert("foo".to_string(), "bar".to_string());
        params.insert("baz".to_string(), "qux".to_string());
        let result = stringify(&params);
        assert!(result.contains("foo=bar"));
        assert!(result.contains("baz=qux"));
    }
}
