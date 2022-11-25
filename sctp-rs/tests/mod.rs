#![cfg(test)]

static TEST_PORT_NO: AtomicU16 = AtomicU16::new(8080);

use sctp_rs::{SctpListener, SctpSocket, SocketToAssociation};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU16, Ordering};

fn create_socket_bind_and_listen(
    association: SocketToAssociation,
    v4: bool,
) -> (SctpListener, SocketAddr) {
    let sctp_socket = if v4 {
        SctpSocket::new_v4(association)
    } else {
        SctpSocket::new_v6(association)
    };
    let port = TEST_PORT_NO.fetch_add(1, Ordering::SeqCst);
    let bindaddr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

    let result = sctp_socket.bind(bindaddr);
    assert!(result.is_ok(), "{:#?}", result.err().unwrap());

    let listener = sctp_socket.listen(10);
    assert!(listener.is_ok(), "{:#?}", listener.err().unwrap());

    (listener.unwrap(), bindaddr)
}

mod connected_socket;
mod listener;
mod socket;
