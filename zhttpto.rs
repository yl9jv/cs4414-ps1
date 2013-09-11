//
// zhttpto.rs
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1

extern mod extra;

use extra::uv;
use extra::{net_ip, net_tcp};
use std::str;
use std::io::file_reader;

static BACKLOG: uint = 5;
static PORT:    uint = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";
static mut counter: uint = 0;

unsafe fn new_connection_callback(new_conn :net_tcp::TcpNewConnection, _killch: std::comm::SharedChan<Option<extra::net_tcp::TcpErrData>>)
{
    do spawn {
        let accept_result = extra::net_tcp::accept(new_conn);
        match accept_result {
            Err(err) => {
               println(fmt!("Connection error: %?", err));
            },  
            Ok(sock) => {
                let peer_addr: ~str = net_ip::format_addr(&sock.get_peer_addr());
                println(fmt!("Received connection from: %s", peer_addr));
                
                let read_result = net_tcp::read(&sock, 0u);
                match read_result {
                    Err(err) => {
                        println(fmt!("Receive error: %?", err));
                    },
                    Ok(bytes) => {
                        let request_str = str::from_bytes(bytes.slice(0, bytes.len() - 1));
                        println(fmt!("Request received:\n%s", request_str));
			counter += 1;
			let mut iter = request_str.split_str_iter("\n");
			let line = iter.nth(0).get();
			let path = line.slice_chars(5, line.len() - 10);
			if path == "" {
                            let response: ~str = ~
                                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                 <doctype !html><html><head><title>Hello, Rust!</title>
                             <style>body { background-color: #111; color: #FFEEAA }
                                    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                             </style></head>
                             <body>
                             <h1>Greetings, Rusty!</h1>\r\n"
			     + "This is request number " 
			     + fmt!("%?", counter) +
                             "</body></html>\r\n";

                        net_tcp::write(&sock, response.as_bytes_with_null_consume());
			}
			else {
			     let filename: ~str = ~"/home/student/cs4414-ps1/" + path.trim();
			     let read_result: Result<@Reader, ~str>;
			     let output: ~str;
			     read_result = file_reader(~PosixPath(filename));

			     match read_result {
				 Ok(file) => 
					output = fmt!("%?", file.read_lines()),
				 Err(err) => 
					output = fmt!("Error reading file: %?", err),			  
			     }
				 let response: ~str = ~
                                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                                 <doctype !html><html><head><title>Hello, Rust!</title>
                             <style>body { background-color: #111; color: #FFEEAA }
                                    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                             </style></head>
                             <body>"
			     + output +
                             "</body></html>\r\n";

                        net_tcp::write(&sock, response.as_bytes_with_null_consume());
			}
                    },
                };
            }
        }
    };
}

fn main() {
    net_tcp::listen(net_ip::v4::parse_addr(IPV4_LOOPBACK), PORT, BACKLOG,
                    &uv::global_loop::get(),
                    |_chan| { println(fmt!("Listening on tcp port %u ...", PORT)); },
                    new_connection_callback);
}
