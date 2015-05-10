extern crate hyper;

use std::env;
use std::net::{SocketAddrV4, Ipv4Addr};

use hyper::{Get};
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri::AbsolutePath;

#[allow(unused_must_use)]
fn index(req: Request, mut res: Response) {
    match req.uri {
        AbsolutePath(ref path) => match (req.method, &path[..]) {
            (Get, "/") => {
                *res.status_mut() = StatusCode::Ok;

                res.send(b"hello, world");

                return;
            },
            _ => {
                *res.status_mut() = StatusCode::NotFound;

                return;
            }

        },
        _ => { return; }
    };
}

#[allow(unused_must_use)]
fn main() {
    let port = match env::var("PORT") {
        Ok(value) => value.parse::<u16>().unwrap(),
        Err(_)    => 3000
    };

    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);

    Server::http(index).listen(addr);
}
