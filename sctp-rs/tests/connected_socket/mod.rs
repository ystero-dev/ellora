use crate::create_socket_bind_and_listen;
use sctp_rs::*;
use std::net::SocketAddr;

#[test]
fn bindx_not_supported() {
    let connected = crate::SctpConnectedSocket::from_rawfd(42);
    let bindaddr = "127.0.0.1:8080".parse().unwrap();
    let result = connected.sctp_bindx(&[bindaddr], crate::BindxFlags::Add);
    assert!(result.is_err(), "{:#?}", result.ok().unwrap());
}
