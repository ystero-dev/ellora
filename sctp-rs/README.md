# Introduction

Initial focus is on implementation of the Sockets Extension API as defined in [RFC 6458](https://www.rfc-editor.org/rfc/rfc6458.txt) that is ergonomic and that should work with any `async` toolkit in Rust. Current implementation is targeted for Linux based systems. These APIs are *not* a wrapper over [`lksctp`](https://lksctp.sourceforge.net/), but instead provide all the APIs that make use of Rust's types like `enum`, `Vec` etc. Eventually it should be possible to have `SCTP` as a first class citizen in the Rust's `async` ecosystem.

In particular this implementation utilizes the `SCTP` stack in the Linux kernel, unlike other approaches like [webrtc-sctp](https://github.com/webrtc-rs/webrtc/tree/master/sctp) which is trying to build entire `SCTP` stack in the user-space and is primarily targeted for running `SCTP` over `DTLS` sockets.

Please see [compatibility](#compatibility) for current feature support.

# Compatibility

## `async` Runtime Supported.
- The implementation supports [Tokio `async` runtime](https://tokio.rs/).

## SCTP Feature Support

This section captures the current support for `SCTP` features with [RFC 6458](https://www.rfc-editor.org/rfc/rfc6458.txt) as a reference. In particular, features marked as `DEPRECATED` in the said RFC are not implemented. Since the Sockets Extension API defined in the RFC describes an API based on C programming language, there is not one to one mapping in the implementation, see notes for further details.

| Section | Compatibility | Notes |
| ---- | ---- | ---- |
| 3.1.1 | yes | |
| 3.1.2 | yes | |
| 3.1.3 | yes | |
| 3.1.4 | yes | See Note 1. |
| 3.1.5 | yes | See Note 2. |
| 3.1.6 | yes | |
| 4.1.1 | yes | |
| 4.1.2 | yes | |
| 4.1.3 | yes | |
| 4.1.4 | yes | |
| 4.1.5 | yes | |
| 4.1.6 | yes | See Note 2. |
| 4.1.7 | yes | |
| 4.1.8 | yes | See Note 1. |
| 4.1.9 | yes | |
| 5.3.1 | yes | |
| 5.3.2 | N/A | |
| 5.3.3 | N/A | |
| 5.3.4 | yes | |
| 5.3.5 | yes | |
| 5.3.6 | yes | See Note 2. |
| 5.3.7 | no | |
| 5.3.8 | no | See Note 1. |
| 5.3.9 | no | |
| 5.3.10 | no | |
| 6.1.1 | yes | |
| 6.1.2 | no | |
| 6.1.3 | no | |
| 6.1.4 | N/A | |
| 6.1.5 | no | |
| 6.1.6 | no | See Note 2. |
| 6.1.7 | no | |
| 6.1.8 | no | See Note 1. |
| 6.1.9 | no | |
| 6.1.10 | no | |
| 6.1.11 | no | |
| 6.2.1 | N/A | |
| 6.2.2 | yes | |
| 8.1.1 | no | |
| 8.1.2 | no | |
| 8.1.3 | no | |
| 8.1.4 | no | |
| 8.1.5 | no | |
| 8.1.6 | no | |
| 8.1.7 | no | |
| 8.1.8 | no | |
| 8.1.9 | no | |
| 8.1.10 | no | |
| 8.1.11 | no | |
| 8.1.12 | no | |
| 8.1.13 | N/A | |
| 8.1.14 | N/A | |
| 8.1.15 | no | |
| 8.1.16 | no | |
| 8.1.17 | no | |
| 8.1.18 | no | |
| 8.1.19 | no | |
| 8.1.20 | no | |
| 8.1.21 | no | |
| 8.1.22 | N/A | |
| 8.1.23 | no | |
| 8.1.24 | no | |
| 8.1.25 | no | |
| 8.1.26 | no | |
| 8.1.27 | no | |
| 8.1.28 | no | |
| 8.1.29 | no | |
| 8.1.30 | no | |
| 8.1.31 | yes | |
| 8.1.32 | no | |
| 8.2.1 | yes | |
| 8.2.2 | no | |
| 8.2.3 | no | |
| 8.2.4 | no | |
| 8.2.5 | no | |
| 8.2.6 | no | |
| 8.3.1 | no | |
| 8.3.2 | no | |
| 8.3.3 | no | |
| 8.3.4 | no | |
| 8.3.5 | no | |
| 9.1 | no | |
| 9.2 | no | |
| 9.3 | no | |
| 9.4 | no | |
| 9.5 | no | |
| 9.6 | no | |
| 9.7 | N/A | |
| 9.8 | N/A | |
| 9.9 | no | See Note 3.|
| 9.10 | N/A | |
| 9.11 | N/A | |
| 9.12 | no | |
| 9.13 | no | |

Notes:
1. The `drop` implementation on the socket 'close'es the socket, hence no explicit `close` call supported.
2. All the Send and Receive functions are available as two APIs `sctp_send` and `sctp_recv`, hence no separate implementation for the C like system calls.
3. This API is not required to be implemented in Rust.
