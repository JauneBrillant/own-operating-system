extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use noli::net::lookup_host;
use noli::net::SocketAddr;
use noli::net::TcpStream;
use saba_core::error::Error;
use saba_core::http::HttpResponse;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        let ips = match lookup_host(&host) {
            Ok(ips) => ips,
            Err(e) => {
                return Err(Error::Network(format!(
                    "Failed to find IP addresses: {:#?}",
                    e
                )))
            }
        };

        if ips.len() < 1 {
            return Err(Error::Network("Failed to find IP adresses".to_string()));
        }

        let socket_addr: SocketAddr = (ips[0], port).into();
        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to connect to TCP stream".to_string(),
                ))
            }
        };

        let request = format!(
            "GET /{} HTTP/1.1\r\nHost: {}\r\nAccept: text/html\r\nConnection: close\r\n\r\n",
            &path, &host
        );

        let _bytes_written = match stream.write(request.as_bytes()) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(Error::Network(
                    "Failed to send a request to TCP stream".to_string(),
                ))
            }
        };

        let mut received = Vec::new();
        loop {
            let mut buf = [0_u8; 4096];
            let bytes_read = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(Error::Network(
                        "Failed to receive a request from TCP stream".to_string(),
                    ))
                }
            };

            if bytes_read == 0 {
                break;
            }

            received.extend_from_slice(&buf[..bytes_read]);
        }

        match core::str::from_utf8(&received) {
            Ok(response) => HttpResponse::new(response.to_string()),
            Err(e) => Err(Error::Network(format!("Invalid received response: {}", e))),
        }
    }
}
