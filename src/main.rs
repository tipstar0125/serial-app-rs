use std::error::Error;
use std::io::prelude::*;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut port = serialport::new("COM1", 9600)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .timeout(Duration::from_millis(100))
        .open()?;


    let mut buf: Vec<u8> = vec![0; 1000];
    loop {
        println!("Write...");
        match port.write("Hello\r\n".as_bytes()) {
            Ok(_) => std::io::stdout().flush()?,
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }

        println!("Read...");
        match port.read(buf.as_mut_slice()) {
            Ok(t) => {
                let bytes = &buf[..t];
                let string = String::from_utf8(bytes.to_vec())?;
                println!("bytes: {:?}", bytes);
                println!("string: {:?}", string);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
        std::thread::sleep(Duration::from_millis(1000));
    }
}
