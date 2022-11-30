//! SCTP Implementation for Rust 'async' runtimes.
//!
//! The Goal of this implementation is being able to use SCTP within Rust's async ecosystem. Also,
//! this implementation is a Pure Rust implementation leveraging the kernel SCTP stack in the Linux
//! kernel without requiring `libsctp` or similar libraries to be present and makes use of the
//! `libc` crate for system calls. This crate implements [Socket API
//! Extensions](https://datatracker.ietf.org/doc/html/rfc6458), that are expected in a modern
//! implementation of SCTP stack. As a result, APIs that are marked as `DEPRECATED` in the RFC will
//! not will not be implemented initially.
//!
//! Also, the APIs are designed such that they are idiomatic Rust APIs and making use of
//! appropriate types thus making use of the [`std::net::SocketAddr`] structures wherever
//! appropriate rather than using the [`libc::sockaddr`] structures for example.

mod connected_socket;
mod listener;
mod socket;

#[doc(inline)]
pub use socket::SctpSocket;

#[doc(inline)]
pub use listener::SctpListener;

#[doc(inline)]
pub use connected_socket::SctpConnectedSocket;

mod internal;

mod consts;

mod types;

#[doc(inline)]
pub use types::{
    AssociationChange, BindxFlags, SctpAssocChangeState, SctpAssociationId, SctpCmsgType,
    SctpEvent, SctpNotification, SctpNotificationOrData, SctpNxtInfo, SctpRcvInfo,
    SctpReceivedData, SctpSendData, SctpSendInfo, SocketToAssociation, SubscribeEventAssocId,
};
