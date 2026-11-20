use sample_rust_core::api::AccountClient;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

#[tokio::test(flavor = "current_thread")]
async fn authenticated_get_me_reads_the_server_profile() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let request: Vec<_> = BufReader::new(&stream)
            .lines()
            .map(Result::unwrap)
            .take_while(|line| !line.is_empty())
            .collect();
        assert_eq!(request[0], "GET /v1/me HTTP/1.1");
        assert!(request.iter().any(|line| {
            line.split_once(": ").is_some_and(|(name, value)| {
                name.eq_ignore_ascii_case("authorization") && value == "Bearer sample"
            })
        }));
        let body =
            sample_rust_core::me_response_json("42".into(), "martin".into(), "Martin".into());
        write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
    });
    let client = AccountClient::new(base_url, "Bearer sample".into());
    let profile = client.get_me().await.unwrap();
    assert_eq!(profile.id, "42");
    assert_eq!(profile.display_name, "Martin");
    server.join().unwrap();
}
