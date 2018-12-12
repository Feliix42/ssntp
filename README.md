# ssntp

The Stupid Simple NTP client.

## About

This is a minimal sntp client based on a task for the lecture [Computer Networks in Practice](https://tu-dresden.de/ing/informatik/sya/professur-fuer-rechnernetze/studium/lehrveranstaltungen/lehrveranstaltungsdetails?ln=de&lv_id=21). It does virtually nothing except for querying a NTP server for the current time and date. As it is only a small implementation to see how NTP works it also does virtually no error checking.

## Building

To build and run the project, clone it and run

```
cargo run -- [ntp server address]
```

Note that you need nightly Rust, as the used conversion method for big endian numbers is only available starting at Rust 1.32.0.
Nightly Rust can be obtained i.e., by using [rustup](https://rustup.rs) by running `rustup override set nightly` in the repository folder.

## License

For licensing information, see the [License file](LICENSE). This work is published under the MIT License.
