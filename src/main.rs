use anyhow::Result;
use httputa::request::RequestMessage;
use std::{
    io::{prelude::*, BufReader, BufWriter},
    net::TcpListener,
};

fn main() -> Result<()> {
    let host = "127.0.0.1";
    let port = "8888";
    let address = format!("{}:{}", host, port);
    let listener = TcpListener::bind(address)?;

    let mut reply_data = String::new();

    for stream in listener.incoming().filter_map(Result::ok) {
        let mut _writer = BufWriter::new(stream.try_clone()?);
        let mut reader = BufReader::new(stream);

        reader.read_line(&mut reply_data)?;
        println!("Data: {}", reply_data);
        println!("Buffer: {}", String::from_utf8(Vec::from(reader.buffer()))?);
        let req = RequestMessage::new(&reply_data)?;
        println!("Request: {:#?}", req);
    }
    Ok(())
}
