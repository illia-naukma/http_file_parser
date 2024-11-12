# HTTP File Parser

## Project Overview
This project is a Rust-based utility for parsing `.http` files, which are commonly used to store HTTP requests in a structured format. By parsing these files, this tool allows automated processing of HTTP requests, useful in scenarios such as API testing.

### Example of an `.http` File

Here’s an example `.http` file that can be parsed by this tool:
```http request
### POST submit request
POST https://eo7nilf42qb5b5e.m.pipedream.net HTTP/1.1
Authorization: Bearer token123
Content-Type: application/json

{ "name": "Alice", "age": 30 }

### GET profile request
GET https://eo7nilf42qb5b5e.m.pipedream.net HTTP/1.1
Accept: application/json

### DELETE account request
DELETE https://eo7nilf42qb5b5e.m.pipedream.net HTTP/1.1
Authorization: Bearer token123

### PUT profile request
PUT https://eo7nilf42qb5b5e.m.pipedream.net HTTP/1.1
Content-Type: application/json

{ "name": "Alice", "age": 31 }

```

## Technical Description

### Parsing Process
Using the Pest library for Rust, this tool analyzes the structure of `.http` files, extracting key information such as:

- **HTTP Method** (e.g., `GET`, `POST`)
- **Request URL** (e.g., `https://example.com/api/resource`)
- **Headers** (e.g., `Content-Type: application/json`)
- **Request Body**

The `.http` files are parsed line-by-line, following a grammar that identifies sections such as the request line, headers, and body. Pest provides a custom grammar defined in `grammar.pest`, which defines these parsing rules in a concise format.

## Grammar
```pest
http_file = { SOI ~ (delimiter | request)* ~ EOI}

request = {
    request_line ~
    headers? ~
    NEWLINE ~
    body?
}

delimiter = { "###" ~ whitespace* ~ title? ~ NEWLINE+ }
title = { (!NEWLINE ~ ANY)+ }

request_line = _{ method ~ " "+ ~ uri ~ " "+ ~ "HTTP/" ~ version ~ NEWLINE }
uri = { (!whitespace ~ ANY)+ }
method = { ("GET" | "DELETE" | "POST" | "PUT") }
version = { (ASCII_DIGIT | ".")+ }
whitespace = _{ " " | "\t" }

headers = { header+ }
header = { header_name ~ ":" ~ whitespace ~ header_value ~ NEWLINE }
header_name = { (!(NEWLINE | ":") ~ ANY)+ }
header_value = { (!NEWLINE ~ ANY)+ }

body = { (!delimiter ~ ANY)+ }
```

### Let's break down the grammar rule by rule:
- ### Overall Structure
The .http file is represented as an http_file, which can contain multiple request blocks, separated by optional delimiter sections. Each request block contains essential components like request_line, headers, and an optional body.
```pest
http_file = { SOI ~ (delimiter | request)* ~ EOI }
```
- SOI: Start of input.
- EOI: End of input.
- The file can contain multiple request and delimiter sections.

- ### Request
Each HTTP request is structured as follows:
```pest
request = {
    request_line ~
    headers? ~
    NEWLINE ~
    body?
}
```
- request_line: Defines the HTTP method, URI, and version.
- headers: Optional section containing one or more headers.
- NEWLINE: Separates headers from the body.
- body: Optional content, can include any data except the delimiter.

- ### Delimiter
The delimiter rule separates different requests within the file and may include an optional title.
```pest
delimiter = { "###" ~ whitespace* ~ title? ~ NEWLINE+ }
title = { (!NEWLINE ~ ANY)+ }
```
- "###": Indicates a delimiter.
- whitespace: Allows for spaces or tabs after the delimiter.
- title: Optional title text for the delimiter section, placed on the same line.

- ### Request Line
Defines the core structure of an HTTP request.
```pest
request_line = _{ method ~ " "+ ~ uri ~ " "+ ~ "HTTP/" ~ version ~ NEWLINE }
```
- method: The HTTP method (e.g., GET, POST).
- uri: The URL endpoint for the request.
- version: The HTTP version (e.g., 1.1).

- ### Method
The allowed HTTP methods:
```pest
method = { ("GET" | "DELETE" | "POST" | "PUT") }
```

- ### URI and Version
Defines the URI and HTTP version formats:
```pest
uri = { (!whitespace ~ ANY)+ }
version = { (ASCII_DIGIT | ".")+ }
```
- uri: Any non-whitespace characters represent the endpoint.
- version: Consists of digits and periods, like 1.1.

- ### Headers
Optional section containing one or more headers.
```pest
headers = { header+ }
header = { header_name ~ ":" ~ whitespace ~ header_value ~ NEWLINE }
header_name = { (!(NEWLINE | ":") ~ ANY)+ }
header_value = { (!NEWLINE ~ ANY)+ }
```
- header_name: The name of the header (e.g., Content-Type).
- header_value: The value of the header (e.g., application/json).

- ### Body
Optional content for the request:
```pest
body = { (!delimiter ~ ANY)+ }
```

### Parsing Outcome and Usage
Once parsed, the extracted data is structured into an `HttpRequest` object. This object is then used to perform HTTP requests via the `reqwest` library and printing the response.
