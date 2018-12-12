use chrono::prelude::*;
use clap::{App, Arg};
use std::net::UdpSocket;

fn get_time(address: &str) -> std::io::Result<()> {
    // multi-purpose buffer for sending/receiving data
    let mut buf = [0u8; 48];
    // set NTP client version 4
    buf[0] = 0x23;

    // bind on any free socket, send packet and retrieve answer
    let sock = UdpSocket::bind("0.0.0.0:0")?;
    sock.send_to(&buf, (address, 123))?;
    sock.recv_from(&mut buf)?;

    // read and extract transmitted time, note that raw_time is based on time since 1900
    let raw_time = u32::from_be_bytes([buf[40], buf[41], buf[42], buf[43]]);
    let current_time = NaiveDateTime::from_timestamp((raw_time - 2208988800) as i64, 0);

    println!(
        "\t Aktuelle Uhrzeit: {}",
        current_time.format("%H:%M:%S (%d.%m.%Y)")
    );

    Ok(())
}

fn main() {
    let matches = App::new("ssntp - The stupid simple NTP client.")
                          .version("0.1")
                          .author("Felix Wittwer <dev@felixwittwer.de>")
                          .about("Retrieves the current time from an NTP server")
                          .arg(Arg::with_name("SERVER")
                               .help("Address of the NTP Server to use")
                               .required(true))
        .get_matches();

    if let Err(e) = get_time(matches.value_of("SERVER").unwrap()) {
        eprintln!("{}", e);
    }
}
