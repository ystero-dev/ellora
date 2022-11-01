//! SCTP Socket: An unconnected SCTP Socket

use std::net::SocketAddr;
use std::os::unix::io::RawFd;

use crate::{BindxFlags, SctpListener, SocketToAssociation};

#[allow(unused)]
use super::internal::*;

/// SCTP Socket: An unconnected SCTP Socket
///
/// When we `listen` on this socket, we get an `SCTPListener` on which we can `accept` to get a
/// `SctpConnectedSocket` (This is like `TCPStream` but since this can have multiple associations
/// (in theory at-least), we are calling it a 'connected' socket.
pub struct SctpSocket {
    inner: RawFd,
}

impl super::__InternalSCTP for SctpSocket {}

impl SctpSocket {
    /// Create a New Socket for IPV4
    pub fn new_v4(assoc: SocketToAssociation) -> Self {
        Self {
            inner: sctp_socket_internal(libc::AF_INET, assoc),
        }
    }

    /// Create a New Socket for IPV6
    pub fn new_v6(assoc: crate::SocketToAssociation) -> Self {
        Self {
            inner: sctp_socket_internal(libc::AF_INET6, assoc),
        }
    }

    /// Bind a socket to a given IP Address
    pub fn bind(&self, addr: SocketAddr) -> std::io::Result<()> {
        self.sctp_bindx(&[addr], BindxFlags::Add)
    }

    /// Listen on a given socket
    pub fn listen(self, backlog: i32) -> std::io::Result<SctpListener> {
        sctp_listen_internal(self.inner, backlog)?;

        Ok(SctpListener::from_raw_fd(self.inner))
    }

    /// Accept on a given socket (valid only for `OneToOne` type sockets
    pub async fn accept(&self) -> std::io::Result<(crate::SctpConnectedSocket, SocketAddr)> {
        unimplemented!();
    }

    /// Connect to a given Server
    pub async fn connect(
        &self,
        _addr: SocketAddr,
    ) -> std::io::Result<(crate::SctpConnectedSocket, SocketAddr)> {
        unimplemented!();
    }

    /// Close the socket
    pub fn close(&self) -> std::io::Result<()> {
        unimplemented!();
    }

    /// Shutdown on the socket
    pub fn shutdown(&self, _how: std::net::Shutdown) -> std::io::Result<()> {
        unimplemented!();
    }

    /// Section 9.1 RFC 6458
    ///
    /// It is possible to call `sctp_bindx` on an  un'bound'.
    pub fn sctp_bindx(&self, addrs: &[SocketAddr], flags: BindxFlags) -> std::io::Result<()> {
        sctp_bindx_internal(self.inner, addrs, flags)
    }
}

#[cfg(test)]
mod tests {

    mod bindx {
        use crate::{BindxFlags, SocketToAssociation};
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

        #[test]
        fn test_bind_success() {
            let sctp_socket = crate::SctpSocket::new_v4(SocketToAssociation::OneToOne);
            let bindaddr = Ipv4Addr::UNSPECIFIED;

            let result = sctp_socket.bind(SocketAddr::new(IpAddr::V4(bindaddr), 0));
            assert!(result.is_ok(), "{:?}", result.err().unwrap());
        }

        #[test]
        fn test_bindx_inaddr_any_add_success() {
            let sctp_socket = crate::SctpSocket::new_v4(SocketToAssociation::OneToOne);
            let bindaddr = Ipv4Addr::UNSPECIFIED;

            let result = sctp_socket
                .sctp_bindx(&[SocketAddr::new(IpAddr::V4(bindaddr), 0)], BindxFlags::Add);
            assert!(result.is_ok(), "{:#?}", result.err().unwrap());
        }

        #[test]
        fn test_bindx_inaddr6_any_add_success() {
            let sctp_socket = crate::SctpSocket::new_v6(SocketToAssociation::OneToOne);
            let bindaddr = Ipv6Addr::UNSPECIFIED;

            let result = sctp_socket
                .sctp_bindx(&[SocketAddr::new(IpAddr::V6(bindaddr), 0)], BindxFlags::Add);
            assert!(result.is_ok(), "{:#?}", result.err().unwrap());
        }

        #[test]
        fn test_bindx_inaddr_any_add_and_remove_failure() {
            let sctp_socket = crate::SctpSocket::new_v6(SocketToAssociation::OneToOne);
            let bindaddr6_localhost = Ipv6Addr::LOCALHOST;

            let result = sctp_socket.sctp_bindx(
                &[SocketAddr::new(IpAddr::V6(bindaddr6_localhost), 8080)],
                BindxFlags::Add,
            );
            assert!(result.is_ok(), "{:#?}", result.err().unwrap());

            let result = sctp_socket.sctp_bindx(
                &[SocketAddr::new(IpAddr::V6(bindaddr6_localhost), 8080)],
                BindxFlags::Remove,
            );
            assert!(result.is_err(), "{:#?}", result.ok().unwrap());
        }
    }
}
