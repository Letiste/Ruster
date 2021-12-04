use std::collections::HashMap;
use std::{net::TcpStream};
use std::io::{prelude::*, BufReader};


pub fn handle_client(mut stream: TcpStream) {
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
      let content = format!("The request was a {} at {}", method, path);
      let response = format!(
          "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
          content.len(),
          content
      );
      stream.write_all(response.as_bytes()).unwrap();
      stream.flush().unwrap();
  }

  let mut headers: HashMap<String, String> = HashMap::new();
  for header in lines.by_ref() {
      if header.is_empty() { break };
      let header: Vec<&str> = header.split(':').collect();
      headers.insert(header[0].trim().to_string(), header[1].trim().to_string());
  }
  println!("The headers are: {:?}", headers);

  let body = lines.fold(String::new(), |body, line| body + &line);
  println!("The body is: {}", body);
}
