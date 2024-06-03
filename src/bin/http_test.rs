#![no_std]
#![no_main]

use alloc::string::{String, ToString};

#[macro_use]
extern crate cirno_user;
#[macro_use]
extern crate alloc;

// http://localhost:6201/ access the http server

use cirno_user::{accept, close, listen, open, read, write};

// get url from the tcp request list.
fn get_url_from_tcp_request(req: &[u8]) -> String {
    let mut index = 0;
    for i in 4..req.len() {
        if req[i] == 0x20 {
            index = i;
            break;
        }
    }

    String::from_utf8_lossy(&req[4..index]).to_string()
}
fn get_page_from_file(file_name:&str) -> String {
    print!("get page from file: {}\n", file_name);
    let mut buf = vec![0u8; 8126];
    let fd = open(file_name, cirno_user::OpenFlags::RDONLY);
    if fd == -1 {
        panic!("Error occurred when opening file");
    }
    let siz = read(fd as usize, &mut buf) as usize;
    close(fd as usize);
    println!("read {} bytes", siz);
    let content = core::str::from_utf8(&buf[..siz as usize]).unwrap();
    content.to_string()
}

// just receive GET requests
fn handle_tcp_client(client_fd: usize) -> bool {
    // a buf to receive the data from the server
    let mut buf = vec![0u8; 1024];

    let len = read(client_fd as usize, &mut buf);

    println!("receive {} bytes", len);
    hexdump(&buf[..len as usize]);

    // verify whether it is a valid HTTP request simply, [0x47,0x45,0x54, 0x20] is GET
    if len < 4 || buf[..4] != [0x47, 0x45, 0x54, 0x20] {
        println!("it's not a valid http request");
        return false;
    }

    let url = get_url_from_tcp_request(&buf);

    if url == "/close" {
        println!("close the server");
        // let fd: isize = open("server_close.html", cirno_user::OpenFlags::RDONLY);
        // if fd == -1 {
        //     panic!("Error occurred when opening file");
        // }
        // let siz = read(fd as usize, &mut page_buf) as usize;
        // close(fd as usize);
        // println!("read {} bytes", siz);
        // let content = core::str::from_utf8(&&page_buf[..siz]).unwrap();
        // get_page_from_file("server_close.html", buf);
        let content = get_page_from_file("server_close.html\0");
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnecion: Close\r\n\r\n{}", content.len(),content);
        write(client_fd, response.as_bytes());
        // terminate the connection immediately.
        return true;
    }
    let content = get_page_from_file("CirnoPage.html\0");
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnecion: Close\r\n\r\n{}", content.len(),content);

    // write a response
    write(client_fd, response.as_bytes());

    false
}

#[no_mangle]
pub fn main() -> i32 {
    println!("This is a very simple http server");

    let tcp_fd = listen(80);

    if tcp_fd < 0 {
        println!("Failed to listen on port 80");
        return -1;
    }

    loop {
        let client = accept(tcp_fd as usize);
        println!("client connected: {}", client);

        if client < 1 {
            println!("Failed to accept a client on port 80");
            return -1;
        }

        if handle_tcp_client(client as usize) {
            break;
        }
    }

    println!("finish tcp test");

    // String::from_utf8_lossy(&buf[..len as usize])

    0
}

#[allow(unused)]
pub fn hexdump(data: &[u8]) {
    const PRELAND_WIDTH: usize = 70;
    println!("{:-^1$}", " hexdump ", PRELAND_WIDTH);
    for offset in (0..data.len()).step_by(16) {
        for i in 0..16 {
            if offset + i < data.len() {
                print!("{:02x} ", data[offset + i]);
            } else {
                print!("{:02} ", "");
            }
        }

        print!("{:>6}", ' ');

        for i in 0..16 {
            if offset + i < data.len() {
                let c = data[offset + i];
                if c >= 0x20 && c <= 0x7e {
                    print!("{}", c as char);
                } else {
                    print!(".");
                }
            } else {
                print!("{:02} ", "");
            }
        }

        println!("");
    }
    println!("{:-^1$}", " hexdump end ", PRELAND_WIDTH);
}
