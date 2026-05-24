// socket

// this file needs to provide functions for
// 1. connecting to a socket
// 2. writing to the socket
// 3. reading from the socket
// 
// at the same time, it should store message ids so it can get the right response back to the
// program. that, or i just hope for the best

use shared::constants;

use std::io;

use tokio::io::Interest;
use tokio::net::UnixStream;

use anyhow::Error;

fn connect_to_socket(path: String) -> Result<UnixStream, Error> {
    let stream = UnixStream::connect(path)?;

    stream
}

fn write_to_socket(stream: UnixStream, msg: Vec<u8>) -> Result<(), Error) {
    let ready = stream.ready(Interest::WRITABLE);

    if ready.is_writable() {
        match stream.try_write(format!("{}\n", msg).as_bytes()) {
            Ok(_) => (),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

fn read_from_socket(stream: UnixStream, msg: String) -> Result<String, Error> {
    let ready = stream.ready(Interest::READABLE);

    if read.is_readable() {
        let mut data = vec![0; 1024];

        match stream.try_read(&mut data) {
            Ok(_) => {
                let out = String::from_utf8(data)?;

                return out;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok("Got no output".to_string())
}
