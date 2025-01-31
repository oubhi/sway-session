use std::env;
use std::error::Error;
use std::io::{ErrorKind, Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

pub fn start_ipc_thread(
    running_flag: Arc<AtomicBool>,
) -> Result<thread::JoinHandle<()>, Box<dyn Error>> {
    println!("Initializing ipc thread");
    // let swaysock = env::var("SWAYSOCK")?;
    let swaysock = match env::var("SWAYSOCK") {
        Ok(s) => s,
        Err(e) => {
            println!("SWAYSOCK ERROR");
            println!("\t{e:?}");
            return Err(Box::new(e));
        }
    };
    let mut stream = match UnixStream::connect(swaysock) {
        Ok(s) => s,
        Err(e) => {
            println!("Socket connect ERROR");
            println!("\t{e:?}");
            return Err(Box::new(e));
        }
    };

    let mut buffer = Vec::from("i3-ipc".as_bytes());
    let subscribe_payload = "[\"shutdown\"]";
    let payload_length = u32::try_from(subscribe_payload.len())?;
    buffer.write_all(&payload_length.to_ne_bytes())?;
    buffer.write_all(&0x2u32.to_ne_bytes())?;
    buffer.write_all(subscribe_payload.as_bytes())?;

    stream.write_all(&buffer)?;

    stream.set_read_timeout(Some(Duration::from_millis(100)))?;

    Ok(thread::spawn(move || {
        println!("Starting ipc thread");
        let mut header: [u8; 14] = [0; 14];
        let mut response: Vec<u8> = Vec::new();
        while running_flag.load(Ordering::SeqCst) {
            match stream.read_exact(&mut header[..]) {
                Ok(_) => {
                    let response_payload_len =
                        u32::from_ne_bytes(header[6..10].try_into().unwrap());
                    let response_payload_type =
                        u32::from_ne_bytes(header[10..14].try_into().unwrap());
                    if response_payload_type == 0x80000006 {
                        running_flag.store(false, Ordering::SeqCst);
                        continue;
                    }
                    println!("Payload type: {response_payload_type}");
                    response.resize(response_payload_len.try_into().unwrap(), 0);
                    stream
                        .read_exact(&mut response[..response_payload_len as usize])
                        .unwrap();
                    println!("Response: {}", String::from_utf8(response.clone()).unwrap());
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => {
                    panic!("{e}")
                }
            };
        }
        println!("Exiting ipc thread");
    }))
}
