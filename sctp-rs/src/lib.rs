//! SCTP Implementation for Rust 'async' runtimes.

// Internal (kind-of) Marker trait `Sctp`. All the types implement this trait, which allows us to
// have commpon implementations for APIs for different socket types/states.
trait __InternalSCTP {}

/// Flags for `sctp_bindx`.
pub enum BindxFlags {
    /// Add the addresses passed (corresponding to `SCTP_BINDX_ADD_ADDR`)
    Add = 0x01,

    /// Remove the addresses passed (corresponding to `SCTP_BINDX_REM_ADDR`)
    Remve = 0x02,
}

/// SocketToAssociation: One-to-Many or One-to-One style Socket
pub enum SocketToAssociation {
    /// One Association per Socket
    OneToOne,

    /// Many Associations per Socket
    OneToMany,
}

mod connected_socket;
mod listener;
mod socket;

#[doc(inline)]
pub use socket::SctpSocket;

#[doc(inline)]
pub use listener::SctpListener;

#[doc(inline)]
pub use connected_socket::SctpConnectedSocket;
