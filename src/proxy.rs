use anyhow::Result;
use http::Uri;
use std::str::FromStr;
use url::Url;

use crate::rewrite::apply_rewrite;
use crate::types::Location;

/// Parse and construct the target URL from a location and request path
pub fn build_target_url(location: &Location, request_path: &str) -> Result<Uri> {
    let mut target_url = Url::parse(&location.target)?;

    // Apply path rewrite if specified
    let path = if let Some(rewrites) = &location.path_rewrite {
        apply_rewrite(request_path, rewrites)
    } else {
        request_path.to_string()
    };

    // Set the path
    target_url.set_path(&path);

    Ok(Uri::from_str(target_url.as_str())?)
}

/// Check if a response is HTML content
pub fn is_html_response(content_type: Option<&str>) -> bool {
    if let Some(ct) = content_type {
        ct.contains("text/html") || ct.contains("application/xhtml")
    } else {
        false
    }
}

/// Inject content into HTML response
pub fn inject_into_html(html: &str, inject_content: &str) -> String {
    if let Some(head_end) = html.find("</head>") {
        let mut result = String::with_capacity(html.len() + inject_content.len());
        result.push_str(&html[..head_end]);
        result.push_str(inject_content);
        result.push_str(&html[head_end..]);
        result
    } else {
        // If no </head>, just append to the beginning
        format!("{}{}", inject_content, html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_target_url() {
        let location = Location {
            rule: "^~ /api".to_string(),
            target: "http://localhost:5000".to_string(),
            inject: None,
            path_rewrite: None,
        };

        let uri = build_target_url(&location, "/api/users").unwrap();
        assert_eq!(uri.to_string(), "http://localhost:5000/api/users");
    }

    #[test]
    fn test_is_html_response() {
        assert!(is_html_response(Some("text/html; charset=utf-8")));
        assert!(is_html_response(Some("application/xhtml+xml")));
        assert!(!is_html_response(Some("application/json")));
        assert!(!is_html_response(None));
    }

    #[test]
    fn test_inject_into_html() {
        let html = "<html><head><title>Test</title></head><body>Content</body></html>";
        let inject = "<script>console.log('injected')</script>";
        let result = inject_into_html(html, inject);
        assert!(result.contains(inject));
        assert!(result.contains("</head>"));
    }
}
