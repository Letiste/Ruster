mod request;
mod response;
mod route_node;
mod router;
pub mod client {
    use crate::request::Request;
    use crate::response::Response;
    use crate::router::Router;
    use std::collections::HashMap;
    use std::io::{prelude::*, BufReader};
    use std::net::TcpStream;
    pub fn handle_client(mut stream: TcpStream) {
        let mut router = Router::new();
        let handler = |_req: Request, _res: Response| println!("HANDLER CALLED");
        router.get("/hello", handler);
        router.post("hello/world", handler);
        let mut reader = BufReader::new(&stream);
        let buffer: Vec<u8> = reader.fill_buf().unwrap().to_vec();
        let mut lines = buffer.lines().map(|l| l.unwrap());
        if let Some(first_line) = lines.next() {
            println!("First line: {}", first_line);
            let request: Vec<&str> = first_line.split(' ').collect();
            let method = request[0];
            println!("Method: {}", method);
            let path = request[1];
            println!("Path: {}", path);
            if let Err(_e) = router.handle_request(&format!("{}{}", method, path)) {
                let response = "HTTP/1.1 404 NOT_FOUND\r\n\r\n".to_string();
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            } else {
                let response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
        let headers = parse_headers(lines.by_ref());
        println!("The headers are: {:?}", headers);

        let body = lines.fold(String::new(), |body, line| body + &line);
        println!("The body is: {}", body);
    }

    fn parse_headers<T>(lines: T) -> HashMap<String, String>
    where
        T: Iterator<Item = String>,
    {
        let mut headers: HashMap<String, String> = HashMap::new();
        for header in lines {
            if header.is_empty() {
                break;
            };
            let header: Vec<&str> = header.split(':').collect();
            headers.insert(header[0].trim().to_string(), header[1].trim().to_string());
        }
        headers
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_correctly_parse_headers() {
            let headers = [
                "Content-Type: application/json".to_string(),
                "Accept: */*".to_string(),
            ];

            let parsed_headers = HashMap::from([
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Accept".to_string(), "*/*".to_string()),
            ]);

            assert_eq!(
                parse_headers(headers.iter().map(|h| h.to_owned())),
                parsed_headers
            );
        }
    }
}
