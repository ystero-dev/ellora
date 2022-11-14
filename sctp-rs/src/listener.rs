//! Listening SCTP Socket

use std::net::SocketAddr;
use std::os::unix::io::{AsRawFd, RawFd};

#[allow(unused)]
use crate::internal::*;
use crate::{types::SctpAssociationId, BindxFlags, SctpConnectedSocket, SctpNotificationOrData};

/// A structure representing a socket that is listening for incoming SCTP Connections.
///
/// This structure is created by an [`crate::SctpSocket`] when it is bound to local address(es)
/// and is waiting for incoming connections by calling the `listen` on the socket. The original
/// [`crate::SctpSocket`] is consumed when this structure is created. See
/// [`crate::SctpSocket::listen`] for more details.
pub struct SctpListener {
    inner: RawFd,
}

impl SctpListener {
    /// Accept on a given socket (valid only for `OneToOne` type sockets
    pub fn accept(&self) -> std::io::Result<(SctpConnectedSocket, SocketAddr)> {
        accept_internal(self.inner)
    }

    /// Shutdown on the socket
    pub fn shutdown(&self, how: std::net::Shutdown) -> std::io::Result<()> {
        shutdown_internal(self.inner, how)
    }

    /// For a `OneToMany` this implicitly accepts (So should return a `SctpConnectedSocket`.)
    pub fn recvmsg(&self) -> std::io::Result<()> {
        unimplemented!();
    }

    /// Binds to one or more local addresses. See: Section 9.1 RFC 6458
    ///
    /// It is possible to call `sctp_bindx` on an already 'bound' (that is 'listen'ing socket.)
    pub fn sctp_bindx(&self, addrs: &[SocketAddr], flags: BindxFlags) -> std::io::Result<()> {
        sctp_bindx_internal(self.inner, addrs, flags)
    }

    /// Peels off a connected SCTP association from the listening socket. See: Section 9.2 RFC 6458
    pub fn sctp_peeloff(
        &self,
        assoc_id: SctpAssociationId,
    ) -> std::io::Result<SctpConnectedSocket> {
        let fd = sctp_peeloff_internal(self.inner, assoc_id)?;
        Ok(SctpConnectedSocket::from_rawfd(fd.as_raw_fd()))
    }

    /// Get Peer Address(es) for the given Association ID. See: Section 9.3 RFC 6458
    ///
    /// This function is supported on the [`SctpListener`] because in the case of One to Many
    /// associations that are not peeled off, we are performing IO operations on the listening
    /// socket itself.
    pub fn sctp_getpaddrs(&self, assoc_id: SctpAssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getpaddrs_internal(self.inner, assoc_id)
    }

    /// Get's the Local Addresses for the association. See: Section 9.4 RFC 6458
    pub fn sctp_getladdrs(&self, assoc_id: SctpAssociationId) -> std::io::Result<Vec<SocketAddr>> {
        sctp_getladdrs_internal(self.inner, assoc_id)
    }

    /// Receive Data or Notification from the listening socket.
    ///
    /// This function returns either a notification or the data.
    pub fn sctp_recvv(&self) -> std::io::Result<SctpNotificationOrData> {
        unimplemented!();
    }

    // functions not part of public APIs
    pub(crate) fn from_raw_fd(fd: RawFd) -> Self {
        Self { inner: fd }
    }
}

impl Drop for SctpListener {
    // Drop for `SctpListener`. We close the `inner` RawFd
    fn drop(&mut self) {
        unsafe { _ = libc::close(self.inner) }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::net::SocketAddr;
    use std::sync::atomic::{AtomicU16, Ordering};

    static TEST_PORT_NO: AtomicU16 = AtomicU16::new(8080);

    fn create_socket_bind_and_listen(
        association: SocketToAssociation,
        v4: bool,
    ) -> (SctpListener, SocketAddr) {
        let sctp_socket = if v4 {
            crate::SctpSocket::new_v4(association)
        } else {
            crate::SctpSocket::new_v6(association)
        };
        let port = TEST_PORT_NO.fetch_add(1, Ordering::SeqCst);
        let bindaddr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

        let result = sctp_socket.bind(bindaddr);
        assert!(result.is_ok(), "{:#?}", result.err().unwrap());

        let listener = sctp_socket.listen(10);
        assert!(listener.is_ok(), "{:#?}", listener.err().unwrap());

        (listener.unwrap(), bindaddr)
    }

    #[test]
    fn listening_sctp_bindx_add_success() {
        let (listener, bindaddr) =
            create_socket_bind_and_listen(SocketToAssociation::OneToOne, true);

        let bindx_bindaddr: SocketAddr = format!("127.0.0.53:{}", bindaddr.port()).parse().unwrap();
        let result = listener.sctp_bindx(&[bindx_bindaddr], BindxFlags::Add);
        assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    }

    #[test]
    fn listening_socket_no_connect_peeloff_failure() {
        let (listener, _) = create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

        let result = listener.sctp_peeloff(42);
        assert!(result.is_err(), "{:#?}", result.ok().unwrap());
    }

    #[ignore]
    #[test]
    fn listening_socket_one2one_connected_peeloff_failure() {
        let (_listener, bindaddr) =
            create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

        let client_socket = SctpSocket::new_v4(SocketToAssociation::OneToOne);
        let assoc_id = client_socket.sctp_connectx(&[bindaddr]);
        assert!(assoc_id.is_ok(), "{:#?}", assoc_id.err().unwrap());
    }

    #[ignore]
    #[test]
    fn listening_socket_one2many_connected_peeloff_success() {
        // TODO Actual test implementation.
        assert!(false);
    }

    #[test]
    fn listening_one_2_one_listen_accept_success() {
        let (listener, bindaddr) =
            create_socket_bind_and_listen(SocketToAssociation::OneToOne, true);

        let client_socket = SctpSocket::new_v4(SocketToAssociation::OneToOne);
        let assoc_id = client_socket.sctp_connectx(&[bindaddr]);
        assert!(assoc_id.is_ok(), "{:#?}", assoc_id.err().unwrap());

        let accept = listener.accept();
        assert!(accept.is_ok(), "{:#?}", accept.err().unwrap());

        // Get Peer Address
        let (accepted, _address) = accept.unwrap();
        let result = accepted.sctp_getpaddrs(0);
        assert!(result.is_ok(), "{:#?}", result.err().unwrap());
    }

    #[test]
    fn listening_one_2_many_listen_accept_failure() {
        let (listener, bindaddr) =
            create_socket_bind_and_listen(SocketToAssociation::OneToMany, true);

        let client_socket = SctpSocket::new_v4(SocketToAssociation::OneToMany);
        let assoc_id = client_socket.sctp_connectx(&[bindaddr]);
        assert!(assoc_id.is_ok(), "{:#?}", assoc_id.err().unwrap());

        let accept = listener.accept();
        assert!(accept.is_err(), "{:#?}", accept.ok().unwrap());
    }
}
