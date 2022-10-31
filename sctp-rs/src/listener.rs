//! Listening SCTP Socket

use std::net::SocketAddr;

/// An `SctpSocket` that is bound to a local address and is 'listen'ing for incoming connections
pub struct SctpListener;

impl super::__InternalSCTP for SctpListener {}

impl SctpListener {
    /// Accept on a given socket (valid only for `OneToOne` type sockets
    pub async fn accept(&self) -> std::io::Result<(crate::SctpConnectedSocket, SocketAddr)> {
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

    /// For a `OneToMany` this implicitly accepts (So should return a `SctpConnectedSocket`.)
    pub fn recvmsg(&self) -> std::io::Result<()> {
        unimplemented!();
    }

    /// Section 9.1 RFC 6458
    ///
    /// It is possible to call `sctp_bindx` on an already 'bound' (that is 'listen'ing socket.)
    pub fn sctp_bindx(
        &self,
        _addrs: &[SocketAddr],
        _flags: crate::BindxFlags,
    ) -> std::io::Result<()> {
        unimplemented!();
    }
}
