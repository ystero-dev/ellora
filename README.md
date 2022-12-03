# Introduction

The goal of the project is to be a toolkit for Stream Control Transport Protocol (`SCTP`) implemented in Rust. Eventually, it should be possible to have `SCTP` supported as well as other transport protocols like `TCP`, `UDP` in the Rust's `async` ecosystem.

Currently following crate is implemented -
* `sctp-rs` - This crate provides ergonomic, safe APIs in Rust. See [README.md](https://github.com/gabhijit/ellora/master/blob/sct-rs/README.md) for more details.

# Motivation

Why this project exists? There are a few crates that support `SCTP` but are either not maintained or do not support Rust's `async` or do not have a well documented APIs (just as wrappers over C library). One major exception being the SCTP support in [webrtc-rs](https://github.com/webrtc-rs/webrtc/tree/master/sctp). However this particular implementation is in active development and using it outside the project is a challenging task (as of now). Further, the support in Linux SCTP stack is matured and for other non WebRTC use cases of `SCTP`, mainly in signaling protocols in the telecom world (3GPP protocols like `S1AP`, `NGAP` use `SCTP` as a transport.), a suitable implementation is useful if not required.
