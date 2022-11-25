//! Types used by the Public APIs

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
    Notification(SctpNotification),

    /// SCTP Data Received by an association.
    Data(SctpReceivedData),
}

/// Structure Representing SCTP Received Data.
///
/// This structure is returned by the `sctp_recv` API call. This contains in addition to 'received'
/// data, any ancillary data that is received during the underlying system call.
#[derive(Debug, Clone)]
pub struct SctpReceivedData {
    pub payload: Vec<u8>,
    pub rcv_info: Option<SctpRcvInfo>,
    pub nxt_info: Option<SctpNxtInfo>,
}

/// Structure Represnting Data to be Sent.
///
/// This structure contains actual paylod and optional ancillary data.
#[derive(Debug, Clone)]
pub struct SctpSendData {
    pub payload: Vec<u8>,
    pub snd_info: Option<SctpSendInfo>,
}

/// Structure representing Ancilliary Send Information (See Section 5.3.4 of RFC 6458)
#[derive(Debug, Default, Clone)]
pub struct SctpSendInfo {
    pub sid: u16,
    pub flags: u16,
    pub ppid: u32,
    pub context: u32,
    pub assoc_id: SctpAssociationId,
}

/// Structure Representing Ancillary Receive Information (See Section 5.3.5 of RFC 6458)
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct SctpRcvInfo {
    pub sid: u16,
    pub ssn: u16,
    pub flags: u16,
    pub ppid: u32,
    pub tsn: u32,
    pub cumtsn: u32,
    pub context: u32,
    pub assoc_id: SctpAssociationId,
}

/// Structure representing Ancillary next information (See Section 5.3.5)
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct SctpNxtInfo {
    pub sid: u16,
    pub flags: u16,
    pub ppid: u32,
    pub length: u32,
    pub assoc_id: SctpAssociationId,
}

#[derive(Debug, Clone)]
pub enum SctpNotification {
    /// Association Change Notification. See Section 6.1.1 of RFC 6458
    AssociationChange(AssociationChange),

    /// A Catchall Notification type for the Notifications that are not supported
    Unsupported,
}

/// AssociationChange: Structure returned as notification for Association Change.
///
/// To subscribe to this notification type, An application should call `sctp_subscribe_event` using
/// the [`SctpEvent`] type as [`SctpEvent::Association`].
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AssociationChange {
    pub assoc_type: u16,
    pub flags: u16,
    pub length: u32,
    pub state: u16,
    pub error: u16,
    pub ob_streams: u16,
    pub ib_streams: u16,
    pub assoc_id: SctpAssociationId,
    pub info: Vec<u8>,
}

/// SctpEvent: Used for Subscribing for SCTP Events
///
/// See [`sctp_subscribe_events`][`crate::SctpListener::sctp_subscribe_event`] for the usage.
#[repr(u16)]
#[derive(Debug, Clone)]
pub enum SctpEvent {
    DataIo = (1 << 15),
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

/// SubscribeEventAssocId: AssociationID Used for Event Subscription
///
/// Note: repr should be same as `SctpAssociationId` (ie. `i32`)
#[repr(i32)]
pub enum SubscribeEventAssocId {
    Future,
    Current,
    All,
    Value(SctpAssociationId),
}

impl From<SubscribeEventAssocId> for SctpAssociationId {
    fn from(value: SubscribeEventAssocId) -> Self {
        match value {
            SubscribeEventAssocId::Future => 0 as Self,
            SubscribeEventAssocId::Current => 1 as Self,
            SubscribeEventAssocId::All => 2 as Self,
            SubscribeEventAssocId::Value(v) => v,
        }
    }
}

/// Association Change States
#[repr(u16)]
pub enum SctpAssocChangeState {
    SctpCommUp = 0,
    SctpCommLost,
    SctpRestart,
    SctpShutdownComplete,
    SctpCantStartAssoc,
}

/// Constants related to `enum sctp_cmsg_type`
#[repr(i32)]
pub enum SctpCmsgType {
    SctpInit = 0,
    SctpSndRcv,
    SctpSndInfo,
    SctpRcvInfo,
    SctpNxtInfo,
    SctpPrInfo,
    SctpAuthInfo,
    SctpDstAddrV4,
    SctpDstAddrV6,
}

pub(crate) mod internal;
