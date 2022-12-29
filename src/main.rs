use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs,
    path::Path
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7171").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request: {:#?}", http_request);
    println!("{}", http_request[0]);
    let get_req:Vec<_> = http_request[0].split_whitespace().collect();
    
    if get_req.len() > 0 
        && Path::new(&get_req[1].replace("/","")).exists() {
        let contents = fs::read_to_string(get_req[1].replace("/",""))
            .expect("Should have been able to read the file");
        println!("{}", http_request[0]);
        println!("{}", contents);

        let status_line = "HTTP/1.1 200 OK";
        let length = contents.len();
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
    else {
        let response = "HTTP/1.1 200 OK\r\n\r\n\r\n";
    
        stream.write_all(response.as_bytes()).unwrap(); 
    }
}
