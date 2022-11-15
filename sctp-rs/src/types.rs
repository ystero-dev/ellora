//! Types used by different SCTP Internal APIs
//!
//! Most of these types are 'C' like `struct`s that are passed as parameters as a part of
//! performing certain SCTP related functionality using `libc::getsockopt` or `libc::setsockopt`.

/// SCTP Association ID Type
pub type SctpAssociationId = i32;

/// Flags used by `sctp_bindx`.
#[derive(Debug, Clone)]
pub enum BindxFlags {
    /// Add the addresses passed (corresponding to `SCTP_BINDX_ADD_ADDR`)
    Add,

    /// Remove the addresses passed (corresponding to `SCTP_BINDX_REM_ADDR`)
    Remove,
}

/// SocketToAssociation: One-to-Many or One-to-One style Socket
#[derive(Debug, Clone)]
pub enum SocketToAssociation {
    /// One Association per Socket
    OneToOne,

    /// Many Associations per Socket
    OneToMany,
}

/// SctpNotificationOrData: A type returned by a `sctp_recvv` call.
#[derive(Debug, Clone)]
pub enum SctpNotificationOrData {
    /// SCTP Notification received by an `sctp_recvv` call
    SctpNotification,

    /// SCTP Data Received by an association.
    SctpData,
}

/// SctpEvent: Used for Subscribing for SCTP Events
///
/// See [`sctp_subscribe_events`][`crate::SctpListener::sctp_subscribe_events`] for the usage.
pub enum SctpEvent {
    DataIo,
    Association,
    Address,
    SendFailure,
    PeerError,
    Shutdown,
    PartialDelivery,
    AdaptationLayer,
    Authentication,
    SenderDry,
    StreamReset,
    AssociationReset,
    StreamChange,
    SendFailureEvent,
}

pub(crate) mod internal;
