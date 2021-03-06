
extern crate shift_it;

use shift_it::http_request::{HttpRequest, HttpTarget};
use shift_it::http_response::{/*HttpResponse, */StatusClass};
use std::io::Read;

#[test]
fn request_bytes_for_a_simple_request() {
    let mut rq = HttpRequest::new("http://www.purple.com/").unwrap();
    rq.add_header("Accept", "text/html");

    let rq_str = String::from_utf8(rq.request_bytes("GET", None)).expect("could not decode string bytes");

    assert_eq!(rq_str, "GET / HTTP/1.1\r\nHost: www.purple.com\r\nConnection: close\r\nContent-Length: 0\r\nAccept: text/html\r\n\r\n");
}

#[test]
fn request_bytes_with_full_query() {
    let rq = HttpRequest::new("https://www.google.co.uk/search?q=term&oq=term&sourceid=chrome&ie=UTF-8#q=term&start=10").unwrap();

    let rq_str = String::from_utf8(rq.request_bytes("GET", None)).expect("could not decode string bytes");

    assert_eq!(rq_str, "GET /search?q=term&oq=term&sourceid=chrome&ie=UTF-8#q=term&start=10 HTTP/1.1\r\nHost: www.google.co.uk\r\nConnection: close\r\nContent-Length: 0\r\n\r\n");
}

#[test]
fn request_bytes_with_multiple_headers() {
    let mut rq = HttpRequest::new("https://me.com").unwrap();
    rq.add_header("Accept", "text/html");
    rq.add_header("Accept", "application/json");
    rq.add_header("Accept", "*/*");
    rq.add_header("User-Agent", "rust");

    let rq_str = String::from_utf8(rq.request_bytes("GET", None)).expect("could not decode string bytes");

    assert_eq!(rq_str, "GET / HTTP/1.1\r\nHost: me.com\r\nConnection: close\r\nContent-Length: 0\r\nAccept: text/html,application/json,*/*\r\nUser-Agent: rust\r\n\r\n");
}

#[test]
fn request_bytes_for_https() {
    let mut rq = HttpRequest::new("https://www.google.co.uk").unwrap();
    rq.add_header("Accept", "text/html");

    let rq_str = String::from_utf8(rq.request_bytes("GET", None)).expect("could not decode string bytes");

    assert_eq!(rq_str, "GET / HTTP/1.1\r\nHost: www.google.co.uk\r\nConnection: close\r\nContent-Length: 0\r\nAccept: text/html\r\n\r\n");
}

#[test]
fn request_targets() {
    let rq_sec_expl = HttpRequest::new("https://google.com:1234/wherever").unwrap().request_target();
    let rq_sec_impl = HttpRequest::new("https://google.com/whatever").unwrap().request_target();
    let rq_uns_expl = HttpRequest::new("http://purple.com:2365").unwrap().request_target();
    let rq_uns_impl = HttpRequest::new("http://purple.com/").unwrap().request_target();

    assert_eq!(rq_sec_expl, HttpTarget::Secure("google.com:1234".to_owned()));
    assert_eq!(rq_sec_impl, HttpTarget::Secure("google.com:443".to_owned()));
    assert_eq!(rq_uns_expl, HttpTarget::Unsecure("purple.com:2365".to_owned()));
    assert_eq!(rq_uns_impl, HttpTarget::Unsecure("purple.com:80".to_owned()));
}

#[test]
fn domain(){
    let domain = HttpRequest::new("https://www.google.com").unwrap().domain();
    assert_eq!(domain, "www.google.com");
}

#[test]
fn very_simple_http_call() {
    let result = shift_it::call_no_data(HttpRequest::new("http://www.purple.com/").unwrap());
    match result {
        Ok(body) => println!("{:?}", body),
        Err(e) => panic!(e)
    };

}

#[test]
fn very_simple_https_call() {
    let mut rq = HttpRequest::new("https://www.google.co.uk").unwrap();
    rq.add_header("Accept", "text/html");

    let result = shift_it::call_no_data(rq);

    match result {
        Ok(body) => println!("{:?}", body),
        Err(e) => panic!(e)
    };
}

#[test]
fn status_code_and_class() {
    let result = shift_it::call_no_data(HttpRequest::new("http://www.purple.com/").unwrap()).unwrap();

    assert_eq!(result.status_code, 200);
    assert_eq!(result.status_class, StatusClass::Success);
}

#[test]
fn iterating_result_body() {
    let result = shift_it::call_no_data(HttpRequest::new("http://www.purple.com/").unwrap()).unwrap();

    for byte in result {
        print!("{}", byte as char);
    }
    println!("");
}

#[test]
fn reading_result_body() {
    let mut result = shift_it::call_no_data(HttpRequest::new("http://www.purple.com/").unwrap()).unwrap();
    let mut body_str = String::new();

    result.read_to_string(&mut body_str).unwrap();

    println!("{}", body_str);
}

#[test]
fn chunked_response() {
    let mut result = shift_it::call_no_data(HttpRequest::new("https://jigsaw.w3.org/HTTP/ChunkedScript").unwrap()).unwrap();
    let mut body_str = String::new();

    result.read_to_string(&mut body_str).unwrap();

    assert!(body_str.starts_with("This output")); // a bad result will have the chunk size instead.

    //println!("{}", body_str);
}

// run this test, and the tests will lock up. Check memory usage with your OS.
// I detected no major leaks.
/*
#[test]
fn leak_test() {
    loop {
        {
            let mut result = shift_it::call_no_data(HttpRequest::new("http://www.purple.com/").unwrap()).unwrap();
            print!("{}", result.next().unwrap());
        }
    }
}*/

