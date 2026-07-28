use crate::types::{Location, RuleType};
use regex::Regex;

/// Parse a location rule string and return the rule type and pattern
fn parse_rule(rule: &str) -> (RuleType, &str) {
    let rule = rule.trim();

    if rule.starts_with("= ") {
        (RuleType::Exact, &rule[2..])
    } else if rule.starts_with("^~ ") {
        (RuleType::PrefixStrict, &rule[3..])
    } else if rule.starts_with("~* ") {
        (RuleType::RegexInsensitive, &rule[3..])
    } else if rule.starts_with("~ ") {
        (RuleType::RegexSensitive, &rule[2..])
    } else {
        (RuleType::Prefix, rule)
    }
}

/// Find the best matching location for a given path
/// Follows nginx location matching priority:
/// 1. Exact match (=)
/// 2. Prefix match with highest priority (^~)
/// 3. Regular expressions in order (~, ~*)
/// 4. Prefix matches in order (/)
pub fn find_match<'a>(locations: &'a [Location], path: &str) -> Option<&'a Location> {
    // 1. Try exact matches
    for loc in locations {
        let (rule_type, pattern) = parse_rule(&loc.rule);
        if rule_type == RuleType::Exact && pattern == path {
            return Some(loc);
        }
    }

    // 2. Try prefix strict matches (^~) - highest priority prefix
    let mut best_strict_prefix: Option<(&'a Location, usize)> = None;
    for loc in locations {
        let (rule_type, pattern) = parse_rule(&loc.rule);
        if rule_type == RuleType::PrefixStrict && path.starts_with(pattern) {
            match best_strict_prefix {
                None => best_strict_prefix = Some((loc, pattern.len())),
                Some((_, len)) if pattern.len() > len => {
                    best_strict_prefix = Some((loc, pattern.len()))
                }
                _ => {}
            }
        }
    }
    if let Some((loc, _)) = best_strict_prefix {
        return Some(loc);
    }

    // 3. Try regex matches in order
    for loc in locations {
        let (rule_type, pattern) = parse_rule(&loc.rule);
        match rule_type {
            RuleType::RegexSensitive => {
                if let Ok(regex) = Regex::new(pattern) {
                    if regex.is_match(path) {
                        return Some(loc);
                    }
                }
            }
            RuleType::RegexInsensitive => {
                if let Ok(regex) = Regex::new(&format!("(?i){}", pattern)) {
                    if regex.is_match(path) {
                        return Some(loc);
                    }
                }
            }
            _ => {}
        }
    }

    // 4. Try prefix matches in order (find longest match)
    let mut best_prefix: Option<(&'a Location, usize)> = None;
    for loc in locations {
        let (rule_type, pattern) = parse_rule(&loc.rule);
        if matches!(rule_type, RuleType::Prefix) && (pattern == "/" || path.starts_with(pattern)) {
            match best_prefix {
                None => best_prefix = Some((loc, pattern.len())),
                Some((_, len)) if pattern.len() > len => {
                    best_prefix = Some((loc, pattern.len()))
                }
                _ => {}
            }
        }
    }

    best_prefix.map(|(loc, _)| loc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        assert_eq!(parse_rule("= /"), (RuleType::Exact, "/"));
        assert_eq!(parse_rule("^~ /api"), (RuleType::PrefixStrict, "/api"));
        assert_eq!(parse_rule("~ \\.js$"), (RuleType::RegexSensitive, "\\.js$"));
        assert_eq!(parse_rule("~* \\.js$"), (RuleType::RegexInsensitive, "\\.js$"));
        assert_eq!(parse_rule("/static"), (RuleType::Prefix, "/static"));
    }

    #[test]
    fn test_exact_match() {
        let locations = vec![
            Location {
                rule: "= /".to_string(),
                target: "http://localhost:8000".to_string(),
                inject: None,
                path_rewrite: None,
            },
            Location {
                rule: "/".to_string(),
                target: "http://localhost:3000".to_string(),
                inject: None,
                path_rewrite: None,
            },
        ];

        let matched = find_match(&locations, "/");
        assert!(matched.is_some());
        assert_eq!(matched.unwrap().target, "http://localhost:8000");
    }

    #[test]
    fn test_prefix_match() {
        let locations = vec![
            Location {
                rule: "^~ /api".to_string(),
                target: "http://localhost:5000".to_string(),
                inject: None,
                path_rewrite: None,
            },
            Location {
                rule: "/".to_string(),
                target: "http://localhost:3000".to_string(),
                inject: None,
                path_rewrite: None,
            },
        ];

        let matched = find_match(&locations, "/api/users");
        assert!(matched.is_some());
        assert_eq!(matched.unwrap().target, "http://localhost:5000");
    }
}
