mod helper;

use std::collections::HashMap;

use helper::remove_char;

/// Parse query string into key value pair
pub fn parse(query: &str) -> HashMap<String, String> {
    let mut hashmap: HashMap<String, String> = HashMap::new();

    // Remove unnecessary part of query '?#!'
    let query_str = remove_char(query, "?#!");

    for query in query_str.split("&") {
        let mut pairs = query.split("="); // Split the key-value pair

        let key = pairs.next().unwrap_or_default();
        let value = pairs.next().unwrap_or_default();

        hashmap.insert(key.to_string(), value.to_string());
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
    fn test_parse() {
        let query = "foo=bar&baz=qux";
        let result = parse(query);

        assert_eq!(result.get("foo"), Some(&"bar".to_string()));
        assert_eq!(result.get("baz"), Some(&"qux".to_string()));
    }

    // #[test]
    // fn test_parse_multiple_value() {
    //     let query = "foo=bar&foo=qux";
    //     let result = parse(query);

    //     assert_eq!(result.get("foo"), Some(&"bar".to_string()));
    //     assert_eq!(result.get("foo"), Some(&"qux".to_string()));
    // }

    #[test]
    fn test_parse_with_leading_character() {
        let query = "?foo=bar&baz=qux";
        let result = parse(query);

        assert_eq!(result.get("foo"), Some(&"bar".to_string()));
        assert_eq!(result.get("baz"), Some(&"qux".to_string()));
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
