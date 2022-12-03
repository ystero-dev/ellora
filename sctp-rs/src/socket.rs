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
/// When we `listen` on this socket, we get an [`crate::SctpListener`] on which we can `accept` to
/// get a [`crate::SctpConnectedSocket`] (This is like `TCPStream` but since this can have multiple
/// associations, we are calling it a 'connected' socket).
pub struct SctpSocket {
    inner: AsyncFd<RawFd>,
}

impl SctpSocket {
    /// Create a New Socket for IPV4
    pub fn new_v4(assoc: SocketToAssociation) -> std::io::Result<Self> {
        Ok(Self {
            inner: AsyncFd::new(sctp_socket_internal(libc::AF_INET, assoc)?)?,
        })
    }

    /// Create a New Socket for IPV6
    pub fn new_v6(assoc: crate::SocketToAssociation) -> std::io::Result<Self> {
        Ok(Self {
            inner: AsyncFd::new(sctp_socket_internal(libc::AF_INET6, assoc)?)?,
        })
    }

    /// Bind a socket to a given IP Address
    pub fn bind(&self, addr: SocketAddr) -> std::io::Result<()> {
        self.sctp_bindx(&[addr], BindxFlags::Add)
    }

    /// Listen on a given socket. Returns [`SctpListener`] consuming this structure.
    pub fn listen(self, backlog: i32) -> std::io::Result<SctpListener> {
        sctp_listen_internal(self.inner, backlog)
    }

    /// Connect to a given Server
    pub async fn connect(
        self,
        addr: SocketAddr,
    ) -> std::io::Result<(SctpConnectedSocket, SctpAssociationId)> {
        sctp_connectx_internal(self.inner, &[addr]).await
    }

    /// Section 9.1 RFC 6458
    ///
    /// It is possible to call `sctp_bindx` on an  un'bound'.
    pub fn sctp_bindx(&self, addrs: &[SocketAddr], flags: BindxFlags) -> std::io::Result<()> {
        sctp_bindx_internal(&self.inner, addrs, flags)
    }

    /// Connect to a multi-homed Peer. See Section 9.9 RFC 6458
    ///
    /// An Unbound socket when connected to a remote end would return a
    /// [`SctpConnectedSocket`][`crate::SctpConnectedSocket`] and an
    /// [`SctpAssociationId`][`crate::types::SctpAssociationId`] tuple.
    pub async fn sctp_connectx(
        self,
        addrs: &[SocketAddr],
    ) -> std::io::Result<(SctpConnectedSocket, SctpAssociationId)> {
        sctp_connectx_internal(self.inner, addrs).await
    }

    /// Event Subscription for the socket.
    pub fn sctp_subscribe_event(
        &self,
        event: SctpEvent,
        assoc_id: SubscribeEventAssocId,
    ) -> std::io::Result<()> {
        sctp_subscribe_event_internal(&self.inner, event, assoc_id, true)
    }

    /// Event Unsubscription for the socket.
    pub fn sctp_unsubscribe_event(
        &self,
        event: SctpEvent,
        assoc_id: SubscribeEventAssocId,
    ) -> std::io::Result<()> {
        sctp_subscribe_event_internal(&self.inner, event, assoc_id, false)
    }

    /// Setup Initiation Message Params
    pub fn sctp_setup_init_params(
        &self,
        ostreams: u16,
        istreams: u16,
        retries: u16,
        timeout: u16,
    ) -> std::io::Result<()> {
        sctp_setup_init_params_internal(&self.inner, ostreams, istreams, retries, timeout)
    }

    /// Request to receive `SctpRcvInfo` ancillary data
    pub fn sctp_request_rcvinfo(&self, on: bool) -> std::io::Result<()> {
        request_rcvinfo_internal(&self.inner, on)
    }

    /// Request to receive `SctpNxtInfo` ancillary data
    pub fn sctp_request_nxtinfo(&self, on: bool) -> std::io::Result<()> {
        request_nxtinfo_internal(&self.inner, on)
    }

    /// Get's the status of the connection associated with the association ID
    pub fn sctp_get_status(&self, assoc_id: SctpAssociationId) -> std::io::Result<SctpStatus> {
        sctp_get_status_internal(&self.inner, assoc_id)
    }
}
