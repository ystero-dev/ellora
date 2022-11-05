//! A Connected SCTP Socket. This is similar to `TCPStream`.

use std::net::SocketAddr;

/// A `ConnectedSctpSocket`
#[derive(Debug)]
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
        Err(std::io::Error::from_raw_os_error(95))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn bindx_not_supported() {
        let connected = crate::SctpConnectedSocket;
        let bindaddr = "127.0.0.1:8080".parse().unwrap();
        let result = connected.sctp_bindx(&[bindaddr], crate::BindxFlags::Add);
        assert!(result.is_err(), "{:#?}", result.ok().unwrap());
    }
}
