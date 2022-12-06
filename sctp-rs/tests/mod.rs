#![cfg(test)]

static TEST_PORT_NO: AtomicU16 = AtomicU16::new(8080);

use sctp_rs::{Listener, Socket, SocketToAssociation};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU16, Ordering};

fn create_socket_bind_and_listen(
    association: SocketToAssociation,
    v4: bool,
) -> (Listener, SocketAddr) {
    let sctp_socket = if v4 {
        Socket::new_v4(association)
    } else {
        Socket::new_v6(association)
    };
    assert!(sctp_socket.is_ok(), "{:#?}", sctp_socket.err().unwrap());
    let sctp_socket = sctp_socket.unwrap();

    let port = TEST_PORT_NO.fetch_add(1, Ordering::SeqCst);
    let bindaddr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

    let result = sctp_socket.bind(bindaddr);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let listener = sctp_socket.listen(10);
    assert!(listener.is_ok(), "{:#?}", listener.err().unwrap());

    (listener.unwrap(), bindaddr)
}

fn create_client_socket(association: SocketToAssociation, v4: bool) -> Socket {
    let client_socket = if v4 {
        Socket::new_v4(association)
    } else {
        Socket::new_v6(association)
    };
    assert!(client_socket.is_ok(), "{:#?}", client_socket.err().unwrap());

    client_socket.unwrap()
}

mod connected_socket;
mod listener;
mod socket;
