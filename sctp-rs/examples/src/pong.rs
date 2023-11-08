//! A simple example demonstrating use of the APIs
//! Sends a Pong to a Ping.
//!

use clap::{Arg, Command};

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    eprintln!("pong");

    let app = Command::new("sctp-rs pong example")
        .author("Abhijit Gadgil <gabhijit@iitbombay.org>")
        .arg(Arg::new("bind").num_args(1).required(true).long("bind"));

    let matches = app.get_matches();

    eprintln!(
        "matches.server: {}",
        matches.get_one::<String>("bind").unwrap()
    );
    let server_address = matches.get_one::<String>("bind").unwrap();
    let server_address: std::net::SocketAddr = server_address.parse().unwrap();

    let server_socket = sctp_rs::Socket::new_v4(sctp_rs::SocketToAssociation::OneToOne)?;
    server_socket.sctp_bindx(&[server_address], sctp_rs::BindxFlags::Add)?;

    let server_socket = server_socket.listen(10)?;

    let (accepted, _client_address) = server_socket.accept().await?;

    loop {
        let received = accepted.sctp_recv().await?;
        if let sctp_rs::NotificationOrData::Data(data) = received {
            eprintln!("received: {:#?}", data);
            if data.payload.is_empty() {
                break;
            }
            let response = format!("pong: {}", String::from_utf8(data.payload).unwrap());
            let send_data = sctp_rs::SendData {
                payload: response.as_bytes().to_vec(),
                snd_info: None,
            };
            accepted.sctp_send(send_data).await?;
        }
    }

    Ok(())
}
