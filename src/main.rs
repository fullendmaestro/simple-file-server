use std::{
    io::{self, Read, Write},
    net::{
        Ipv4Addr, SocketAddr, TcpListener, TcpStream
    }
};
use simple_file_server::http::{request, response};


fn create_socket() -> SocketAddr {
    SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::LOCALHOST), 5500)
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let mut stream = stream; // Convert to mutable

    // Read data from the stream
    stream.read(&mut buffer)?;

    // Convert buffer to string and parse the HTTP request
    let buf_str = String::from_utf8_lossy(&buffer);
    let request = request::HttpRequest::new(&buf_str)?;
    let response = request.response()?;

    // Print response for debugging
    println!("{:?}", response);
    println!("{}", response.response_body);

    // Write response to the stream
    stream.write_all(response.response_body.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn serve(socket: SocketAddr) -> io::Result<()> {
    let listener = TcpListener::bind(socket)?;
    let mut counter = 0;
    for stream in listener.incoming() {
        let stream = stream?; // Extract the TcpStream

        std::thread::spawn(move || {
            if let Err(e) = handle_client(stream) {
                eprintln!("Error handling client: {}", e);
            }
        });

        counter += 1;
        println!("Connected stream... {}", counter);
    }
    Ok(())
}


fn main() -> io::Result<()> {
    let socket = create_socket();
    serve(socket)?;
    Ok(())
}