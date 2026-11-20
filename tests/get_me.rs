use sample_rust_core::api::AccountClient;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn spring_get_mapping_path<'a>(filename: &str, readme: &'a str) -> &'a str {
    let mut mappings = readme
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("@GetMapping("));
    let mapping = mappings
        .next()
        .unwrap_or_else(|| panic!("Expected a Spring @GetMapping in {filename}"));
    assert!(
        mappings.next().is_none(),
        "Expected exactly one Spring @GetMapping in {filename}"
    );
    mapping
        .trim_start()
        .strip_prefix('"')
        .and_then(|argument| argument.split_once('"'))
        .map(|(path, _)| path)
        .filter(|path| path.starts_with('/'))
        .unwrap_or_else(|| {
            panic!("Expected a quoted path as the first @GetMapping argument in {filename}")
        })
}

#[tokio::test(flavor = "current_thread")]
async fn authenticated_get_me_matches_the_spring_route_and_reads_the_profile() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let request: Vec<_> = BufReader::new(&stream)
            .lines()
            .map(Result::unwrap)
            .take_while(|line| !line.is_empty())
            .collect();
        assert!(request.iter().any(|line| {
            line.split_once(": ").is_some_and(|(name, value)| {
                name.eq_ignore_ascii_case("authorization") && value == "Bearer sample"
            })
        }));
        let body =
            sample_rust_core::me_response_json("42".into(), "martin".into(), "Martin".into());
        write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
        request
            .into_iter()
            .next()
            .expect("Expected an HTTP request")
    });
    let client = AccountClient::new(base_url, "Bearer sample".into());
    let profile = client.get_me().await.unwrap();
    assert_eq!(profile.id, "42");
    assert_eq!(profile.display_name, "Martin");
    let request_line = server.join().unwrap();
    for (filename, readme) in [
        ("README.md", include_str!("../README.md")),
        ("README.ko.md", include_str!("../README.ko.md")),
    ] {
        let path = spring_get_mapping_path(filename, readme);
        assert_eq!(
            request_line,
            format!("GET {path} HTTP/1.1"),
            "The Rust request must match the Spring route in {filename}"
        );
    }
}
