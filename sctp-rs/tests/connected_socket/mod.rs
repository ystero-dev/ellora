use sctp_rs::*;

#[tokio::test]
async fn bindx_not_supported() {
    let connected = SctpConnectedSocket::from_rawfd(42);
    assert!(connected.is_err(), "{:?}", connected.ok().unwrap());

    // TODO: Write real test
}
