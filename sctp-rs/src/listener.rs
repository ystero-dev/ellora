//! Listening SCTP Socket

use std::net::SocketAddr;
use std::os::unix::io::RawFd;

use crate::internal::sctp_bindx_internal;
use crate::BindxFlags;

/// An `SctpSocket` that is bound to a local address and is 'listen'ing for incoming connections
pub struct SctpListener {
    inner: RawFd,
}

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
    pub fn sctp_bindx(&self, addrs: &[SocketAddr], flags: BindxFlags) -> std::io::Result<()> {
        sctp_bindx_internal(self.inner, addrs, flags)
    }

    // functions not part of public APIs
    pub(crate) fn from_raw_fd(fd: RawFd) -> Self {
        Self { inner: fd }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::net::SocketAddr;

    #[test]
    fn listening_sctp_bindx_add_success() {
        let sctp_socket = crate::SctpSocket::new_v6(SocketToAssociation::OneToOne);
        let bindaddr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

        let result = sctp_socket.bind(bindaddr);
        assert!(result.is_ok(), "{:#?}", result.err().unwrap());

        let listener = sctp_socket.listen(10);
        assert!(listener.is_ok(), "{:#?}", listener.err().unwrap());

        let listener = listener.unwrap();
        let bindaddr = "127.0.0.53:8080".parse().unwrap();
        let result = listener.sctp_bindx(&[bindaddr], BindxFlags::Add);
        assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    }
}
