//! A Connected SCTP Socket. This is similar to `TCPStream`.

use std::net::SocketAddr;

/// A `ConnectedSctpSocket`
pub struct SctpConnectedSocket;

impl super::__InternalSCTP for SctpConnectedSocket {}

impl SctpConnectedSocket {
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
    /// Since this is an optional feature, this should simply return an error!
    pub fn sctp_bindx(
        &self,
        _addrs: &[SocketAddr],
        _flags: crate::BindxFlags,
    ) -> std::io::Result<()> {
        unimplemented!();
    }
}
