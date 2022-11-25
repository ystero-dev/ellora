use sctp_rs::*;

#[test]
fn bindx_not_supported() {
    let connected = SctpConnectedSocket::from_rawfd(42);
    let bindaddr = "127.0.0.1:8080".parse().unwrap();
    let result = connected.sctp_bindx(&[bindaddr], BindxFlags::Add);
    assert!(result.is_err(), "{:#?}", result.ok().unwrap());
}
