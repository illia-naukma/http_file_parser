#[cfg(test)]
mod tests {
    use http_file_parser::parser::HttpParser;
    use http_file_parser::parser::Rule;
    use pest::Parser;

    fn parse_rule(
        rule: Rule,
        input: &str,
    ) -> Result<pest::iterators::Pairs<Rule>, pest::error::Error<Rule>> {
        HttpParser::parse(rule, input)
    }

    #[test]
    fn test_http_file() {
        let input = "GET / HTTP/1.1\nHost: example.com\n\n### My Title\n\nPOST /submit HTTP/1.1\nContent-Type: text/plain\n\nBody content";
        assert!(parse_rule(Rule::http_file, input).is_ok());
    }

    #[test]
    fn test_request() {
        let input = "GET / HTTP/1.1\nHost: example.com\n\n";
        assert!(parse_rule(Rule::request, input).is_ok());
    }

    #[test]
    fn test_delimiter() {
        let input = "### My Title\n\n";
        assert!(parse_rule(Rule::delimiter, input).is_ok());
    }

    #[test]
    fn test_title() {
        let input = "My Title";
        assert!(parse_rule(Rule::title, input).is_ok());
    }

    #[test]
    fn test_request_line() {
        let input = "GET / HTTP/1.1\n";
        assert!(parse_rule(Rule::request_line, input).is_ok());
    }

    #[test]
    fn test_uri() {
        let input = "/path/to/resource";
        assert!(parse_rule(Rule::uri, input).is_ok());
    }

    #[test]
    fn test_method() {
        let methods = ["GET", "DELETE", "POST", "PUT"];
        for method in methods {
            assert!(parse_rule(Rule::method, method).is_ok());
        }
    }

    #[test]
    fn test_version() {
        let input = "1.1";
        assert!(parse_rule(Rule::version, input).is_ok());
    }

    #[test]
    fn test_headers() {
        let input = "Host: example.com\nContent-Type: text/plain\n";
        assert!(parse_rule(Rule::headers, input).is_ok());
    }

    #[test]
    fn test_header() {
        let input = "Host: example.com\n";
        assert!(parse_rule(Rule::header, input).is_ok());
    }

    #[test]
    fn test_header_name() {
        let input = "Content-Type";
        assert!(parse_rule(Rule::header_name, input).is_ok());
    }

    #[test]
    fn test_header_value() {
        let input = "text/plain";
        assert!(parse_rule(Rule::header_value, input).is_ok());
    }

    #[test]
    fn test_body() {
        let input = "This is the body content.";
        assert!(parse_rule(Rule::body, input).is_ok());
    }

    #[test]
    fn test_whitespace() {
        let input = " ";
        assert!(parse_rule(Rule::whitespace, input).is_ok());
    }
}
