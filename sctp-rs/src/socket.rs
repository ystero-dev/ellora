//! SCTP Socket: An unconnected SCTP Socket

use std::net::SocketAddr;
use std::os::unix::io::RawFd;

/// SCTP Socket: An unconnected SCTP Socket
///
/// When we `listen` on this socket, we get an `SCTPListener` on which we can `accept` to get a
/// `SctpConnectedSocket` (This is like `TCPStream` but since this can have multiple associations
/// (in theory at-least), we are calling it a 'connected' socket.
pub struct SctpSocket {
    _inner: RawFd,
}

impl super::__InternalSCTP for SctpSocket {}

impl SctpSocket {
    /// Create a New Socket for IPV4
    pub fn new_v4(_assoc: crate::SocketToAssociation) -> Self {
        unimplemented!();
    }

    /// Create a New Socket for IPV6
    pub fn new_v6(_assoc: crate::SocketToAssociation) -> Self {
        unimplemented!();
    }

    /// Bind a socket to a given IP Address
    pub fn bind(&self, _addr: SocketAddr) -> std::io::Result<()> {
        unimplemented!();
    }

    /// Listen on a given socket
    pub fn listen(self, _backlog: u32) -> std::io::Result<crate::SctpListener> {
        unimplemented!();
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
    pub fn sctp_bindx(
        &self,
        _addrs: &[SocketAddr],
        _flags: crate::BindxFlags,
    ) -> std::io::Result<()> {
        unimplemented!();
    }
}
