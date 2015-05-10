extern crate hyper;

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
    Server::http(index).listen("0.0.0.0:3000");
}
