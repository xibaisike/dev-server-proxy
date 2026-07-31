use regex::Regex;
use std::collections::HashMap;

/// Apply path rewrite rules to a given path
/// Returns the rewritten path
pub fn apply_rewrite(
    path: &str,
    rules: &HashMap<String, String>,
) -> String {
    let mut result = path.to_string();

    for (pattern, replacement) in rules {
        if let Ok(regex) = Regex::new(pattern) {
            result = regex.replace_all(&result, replacement.as_str()).to_string();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_rewrite() {
        let mut rules = HashMap::new();
        rules.insert("^/api".to_string(), "".to_string());

        let result = apply_rewrite("/api/users", &rules);
        assert_eq!(result, "/users");
    }

    #[test]
    fn test_multiple_rewrites() {
        let mut rules = HashMap::new();
        rules.insert("^/v1".to_string(), "/api/v1".to_string());

        let result = apply_rewrite("/v1/users", &rules);
        assert_eq!(result, "/api/v1/users");
    }
}
