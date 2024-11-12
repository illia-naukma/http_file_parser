#[cfg(test)]
mod tests {
    use http_file_parser::parser::parse;

    #[test]
    fn test_valid_request() {
        let input = r#"### Some valid request
POST https://example.com/api/resource HTTP/1.1
Content-Type: application/json
Authorization: Bearer some_token

{"key": "value"}"#;

        let parsed = parse(input).unwrap();
        assert_eq!(parsed.requests.len(), 1);
        let request = &parsed.requests[0];
        assert_eq!(request.method.to_string(), "POST");
        assert_eq!(request.url, "https://example.com/api/resource");
        assert_eq!(request.version, "1.1");
        assert_eq!(request.body, r#"{"key": "value"}"#);
        assert!(request.headers.contains_key("Content-Type"));
        assert_eq!(request.headers["Content-Type"], "application/json");
    }

    #[test]
    fn test_get_request() {
        let input = r#"### Some GET request
GET /api/resource HTTP/1.1
Accept: application/json

"#;

        let parsed = parse(input).unwrap();
        assert_eq!(parsed.requests.len(), 1);
        let request = &parsed.requests[0];
        assert_eq!(request.method.to_string(), "GET");
        assert_eq!(request.url, "/api/resource");
        assert_eq!(request.version, "1.1");
        assert_eq!(request.body, "");
        assert!(request.headers.contains_key("Accept"));
        assert_eq!(request.headers["Accept"], "application/json");
    }

    #[test]
    fn test_multiple_requests() {
        let input = r#"### Request
GET /api/resource HTTP/1.1
Accept: application/json

### Another request

POST /api/resource HTTP/1.1
Content-Type: application/json

{"key": "value"}"#;

        let parsed = parse(input).unwrap();
        assert_eq!(parsed.requests.len(), 2);

        let get_request = &parsed.requests[0];
        assert_eq!(get_request.method.to_string(), "GET");
        assert_eq!(get_request.url, "/api/resource");

        let post_request = &parsed.requests[1];
        assert_eq!(post_request.method.to_string(), "POST");
        assert_eq!(post_request.url, "/api/resource");
        assert_eq!(post_request.body, r#"{"key": "value"}"#);
    }

    #[test]
    fn test_empty_body() {
        let input = r#"### Request with empty body
GET /api/resource HTTP/1.1
Accept: application/json

"#;

        let parsed = parse(input).unwrap();
        let request = &parsed.requests[0];
        assert_eq!(request.body, "");
    }

    #[test]
    fn test_delimiter() {
        let input = r#"### Request
GET /api/resource HTTP/1.1
Accept: application/json

### Another request

POST /api/resource HTTP/1.1
Content-Type: application/json

{"key": "value"}"#;

        let parsed = parse(input).unwrap();
        assert_eq!(parsed.requests.len(), 2);
    }

    #[test]
    fn test_headers_parsing() {
        let input = r#"### Request with headers
POST /api/resource HTTP/1.1
Content-Type: application/json
Authorization: Bearer token

"#;

        let parsed = parse(input).unwrap();
        let request = &parsed.requests[0];
        assert!(request.headers.contains_key("Content-Type"));
        assert_eq!(request.headers["Content-Type"], "application/json");
        assert!(request.headers.contains_key("Authorization"));
        assert_eq!(request.headers["Authorization"], "Bearer token");
    }
}
