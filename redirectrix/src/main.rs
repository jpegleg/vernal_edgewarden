use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const MAX_REQUEST_SIZE: usize = 8192;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:80")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    let _ = handle(stream);
                });
            }
            Err(_) => {}
        }
    }

    Ok(())
}

fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; MAX_REQUEST_SIZE];
    let n = stream.read(&mut buf)?;
    if n == 0 {
        return Ok(());
    }

    let req = match std::str::from_utf8(&buf[..n]) {
        Ok(s) => s,
        Err(_) => {
            return respond(&mut stream, 400, "Bad Request", None);
        }
    };

    let mut lines = req.split("\r\n");

    let request_line = match lines.next() {
        Some(line) => line,
        None => return respond(&mut stream, 400, "Bad Request", None),
    };

    let mut parts = request_line.split_whitespace();
    let _method = parts.next();
    let path = match parts.next() {
        Some(p) if p.starts_with('/') => p,
        _ => "/",
    };

    let host = lines
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            if name.eq_ignore_ascii_case("host") {
                Some(value.trim())
            } else {
                None
            }
        })
        .and_then(sanitize_host);

    let location = match host {
        Some(host) => format!("https://{}{}", host, path),
        None => String::from("https:///"),
    };

    respond(&mut stream, 308, "Permanent Redirect", Some(&location))
}

fn sanitize_host(host: &str) -> Option<String> {
    if host.is_empty() || host.len() > 255 {
        return None;
    }

    if host.contains('\r') || host.contains('\n') || host.contains('/') || host.contains('\\') {
        return None;
    }

    if let Some(stripped) = host.strip_suffix(":80") {
        if !stripped.is_empty() {
            return Some(stripped.to_string());
        }
    }

    Some(host.to_string())
}

fn respond(
    stream: &mut TcpStream,
    code: u16,
    reason: &str,
    location: Option<&str>,
) -> std::io::Result<()> {
    let mut response = format!(
        "HTTP/1.1 {} {}\r\n\
         Connection: close\r\n\
         Content-Length: 0\r\n",
        code, reason
    );

    if let Some(location) = location {
        response.push_str("Location: ");
        response.push_str(location);
        response.push_str("\r\n");
    }

    response.push_str("\r\n");
    stream.write_all(response.as_bytes())?;
    stream.flush()
}
