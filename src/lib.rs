mod helper;

use std::collections::HashMap;

use helper::remove_char;

/// Parse query string into key value pair
pub fn parse(query: &str) -> HashMap<String, String> {
    // Remove unnecessary part of query '?#!'
    let query = remove_char(query, "?#!");

    query
        .split("&")
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, "=");
            let key = parts.next()?; // Get the key
            let value = parts.next().unwrap_or(""); // Get the value

            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let query = "foo=bar&baz=qux";
        let query_1 = "?foo=bar&baz=qux";
        let result = parse(query);
        let result_1 = parse(query_1);

        assert_eq!(result.get("foo"), Some(&"bar".to_string()));
        assert_eq!(result.get("baz"), Some(&"qux".to_string()));
        assert_eq!(result_1.get("foo"), Some(&"bar".to_string()));
        assert_eq!(result_1.get("baz"), Some(&"qux".to_string()));
    }
}
