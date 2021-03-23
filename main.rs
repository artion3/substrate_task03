//get access to certain traits that let us read from and write to the stream
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
//listen at the address 127.0.0.1:7878 for incoming TCP connection, 7878 is the port 
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
// 'incoming' trturns an iterator that gives us a sequence of streams (TcpStream) 
    
// for loop will process each connection in turn and produce a series of streams to handle
    for stream in listener.incoming() {
    
        match stream {
// call the new handle_connection function and pass the stream to the connection
// if correct
        Ok(stream) => {
           handle_connection(stream);
                      }
// if error                      
        Err(error) => {
           println! ("Problem with connection: {}", error)
                      }
                  }
               }
         }
         
// stream parameter mutable, reason :TcpStream instance keeps track of what data it returns to us internally, because its internal state might change
fn handle_connection(mut stream: TcpStream) {
//  declare a buffer on the stack to hold the data that is read in, 1024 bytes in size
    let mut buffer = [0; 1024];
//  pass the buffer to stream.read, which will read bytes from the TcpStream and put them in the buffer.
    stream.read(&mut buffer).unwrap();
      
    // example : let response =  "TCP/1.1 200 OK\r\n\r\n";
    
    let response = String::from_utf8_lossy(&buffer[..]);

//  convert the bytes in the buffer to a string and print
    println!("Request: {}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}



