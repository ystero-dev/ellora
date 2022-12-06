//! SCTP Socket: An unconnected SCTP Socket

use std::net::SocketAddr;
use std::os::unix::io::RawFd;

use tokio::io::unix::AsyncFd;

use crate::{
    BindxFlags, SctpAssociationId, SctpConnectedSocket, SctpEvent, SctpListener, SctpStatus,
    SocketToAssociation, SubscribeEventAssocId,
};

#[allow(unused)]
use super::internal::*;

/// A structure representing and unconnected SCTP Socket.
///
/// When we `listen` on this socket, we get an [`SctpListener`] on which we can `accept` to
/// get a [`SctpConnectedSocket`] (This is like `TCPStream` but since this can have multiple
/// associations, we are calling it a 'connected' socket).
pub struct SctpSocket {
    inner: AsyncFd<RawFd>,
}

impl SctpSocket {
    /// Create a New IPv4 family socket.
    ///
    /// [`SocketToAssociation`] determines the type of the socket created. For a TCP style
    /// socket use [`OneToOne`][`SocketToAssociation::OneToOne`] and for a UDP style socket use
    /// [`OneToMany`][`SocketToAssociation::OneToMany`]. The socket created is set to a
    /// non-blocking socket and is registered for polling for read-write events.
    /// For any potentially blocking I/O operations, whether the socket is 'readable' or
    /// 'writable' is handled internally.
    pub fn new_v4(assoc: SocketToAssociation) -> std::io::Result<Self> {
        Ok(Self {
            inner: AsyncFd::new(sctp_socket_internal(libc::AF_INET, assoc)?)?,
        })
    }

    /// Create a New IPv6 family socket.
    ///
    /// [`SocketToAssociation`] determines the type of the socket created. For a TCP style
    /// socket use [`SocketToAssociation::OneToOne`] and for a UDP style socket use
    /// [`SocketToAssociation::OneToMany`]. The socket created is set to a non-blocking
    /// socket and is registered for polling for read-write events. For any potentially blocking
    /// I/O operations, whether the socket is 'readable' or 'writable' is handled internally.
    pub fn new_v6(assoc: SocketToAssociation) -> std::io::Result<Self> {
        Ok(Self {
            inner: AsyncFd::new(sctp_socket_internal(libc::AF_INET6, assoc)?)?,
        })
    }

    /// Bind a socket to a given IP Address.
    ///
    /// The passed IP address can be an IPv4 or an IPv6, IP address. For the IPv6 family sockets,
    /// it is possible to bind to both IPv4 and IPv6 addresses. IPv4 family sockets can be bound
    /// only to IPv4 addresses only.
    pub fn bind(&self, addr: SocketAddr) -> std::io::Result<()> {
        self.sctp_bindx(&[addr], BindxFlags::Add)
    }

    /// Listen on a given socket.
    ///
    /// This successful operation  returns [`SctpListener`] consuming this structure. The `backlog`
    /// parameter determines the length of the listen queue.
    pub fn listen(self, backlog: i32) -> std::io::Result<SctpListener> {
        sctp_listen_internal(self.inner, backlog)
    }

    /// Connect to SCTP Server.
    ///
    /// The successful operation returns [`SctpConnectedSocket`] consuming this structure.
    pub async fn connect(
        self,
        addr: SocketAddr,
    ) -> std::io::Result<(SctpConnectedSocket, SctpAssociationId)> {
        sctp_connectx_internal(self.inner, &[addr]).await
    }

    /// SCTP Specific extension for binding to multiple addresses on a given socket. See Section
    /// 9.1 RFC 6458.
    ///
    /// `sctp_bindx` API can be used to add or remove additional addresses to an unbound (ie newly
    /// created socket) or a socket that is already bound to address(es) (flag
    /// [`Add`][`BindxFlags::Add`]).  It is also possible to 'remove' bound addresses from the
    /// socket using the same API (flag [`Remove`][`BindxFlags::Remove`]). See the section 9.1
    /// for more details about the semantics of which addresses are acceptable for addition or
    /// removoal using the `sctp_bindx` API.
    pub fn sctp_bindx(&self, addrs: &[SocketAddr], flags: BindxFlags) -> std::io::Result<()> {
        sctp_bindx_internal(&self.inner, addrs, flags)
    }

    /// Connect to a multi-homed Peer. See Section 9.9 RFC 6458
    ///
    /// An Unbound socket when connected to a remote end would return a tuple containing a
    /// [connected socket][`SctpConnectedSocket`] and an [associaton ID][`SctpAssociationId`]. In
    /// the case of One-to-many sockets, this association ID can be used for subscribing to SCTP
    /// events and requesting additional anciliary control data on the socket.
    pub async fn sctp_connectx(
        self,
        addrs: &[SocketAddr],
    ) -> std::io::Result<(SctpConnectedSocket, SctpAssociationId)> {
        sctp_connectx_internal(self.inner, addrs).await
    }

    /// Subscribe to a given SCTP Event on the given socket. See section 6.2.1 of RFC6458.
    ///
    /// SCTP allows receiving notifications about the changes to SCTP associations etc from the
    /// user space. For these notification events to be received, this API is used to subsribe for
    /// the events while receiving the data on the SCTP Socket.
    pub fn sctp_subscribe_event(
        &self,
        event: SctpEvent,
        assoc_id: SubscribeEventAssocId,
    ) -> std::io::Result<()> {
        sctp_subscribe_event_internal(&self.inner, event, assoc_id, true)
    }

    /// Unsubscribe from a given SCTP Event on the given socket. See section 6.2.1 of RFC6458.
    ///
    /// See [`sctp_subscribe_event`][`Self::sctp_subscribe_event`] for further details.
    pub fn sctp_unsubscribe_event(
        &self,
        event: SctpEvent,
        assoc_id: SubscribeEventAssocId,
    ) -> std::io::Result<()> {
        sctp_subscribe_event_internal(&self.inner, event, assoc_id, false)
    }

    /// Setup parameters for a new association.
    ///
    /// To specify custom parameters for a new association this API is used.
    pub fn sctp_setup_init_params(
        &self,
        ostreams: u16,
        istreams: u16,
        retries: u16,
        timeout: u16,
    ) -> std::io::Result<()> {
        sctp_setup_init_params_internal(&self.inner, ostreams, istreams, retries, timeout)
    }

    /// Request to receive `SctpRcvInfo` ancillary data.
    ///
    /// SCTP allows receiving ancillary data about the curent data received on the given socket.
    /// This API is used to obtain receive side additional info when the data is to be received.
    pub fn sctp_request_rcvinfo(&self, on: bool) -> std::io::Result<()> {
        request_rcvinfo_internal(&self.inner, on)
    }

    /// Request to receive `SctpNxtInfo` ancillary data.
    ///
    /// SCTP allows receiving ancillary data about the curent data received on the given socket.
    /// This API is used to obtain information about the next datagram that will be received.
    pub fn sctp_request_nxtinfo(&self, on: bool) -> std::io::Result<()> {
        request_nxtinfo_internal(&self.inner, on)
    }

    /// Get the status of the connection associated with the association ID.
    pub fn sctp_get_status(&self, assoc_id: SctpAssociationId) -> std::io::Result<SctpStatus> {
        sctp_get_status_internal(&self.inner, assoc_id)
    }
}
