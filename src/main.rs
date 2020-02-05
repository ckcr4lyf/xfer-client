use std::net::TcpStream;
use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::time::Instant;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: ./xfer-client [ip] [port] [filename]");
        return;
    }

    let path = &args[3];
    let mut file =  match File::open(path){
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open file.");
            return;
        }
    };

    let ssize = file.metadata().unwrap().len();

    let ip = &args[1];
    let port = &args[2];
    let mut connection_string = String::new();
    connection_string.push_str(ip);
    connection_string.push_str(":");
    connection_string.push_str(port);
    //Read the file into a vector buffer

    let filename_size = path.len() as u16;
    let header_hex = filename_size.to_be_bytes();
    
    // file.read(&mut buf).unwrap();
    // println!("{:X}",buf[0]);
    let tcp_stream = TcpStream::connect(connection_string);
    let mut stream = match tcp_stream {
        Ok(stream) => stream,
        Err(_) => {
            println!("Failed to connect to server. Exiting...");
            return;
        }
    };

    let final_header = [&header_hex, path.as_bytes()].concat();
    // final_header.push(path.as_bytes());

    match stream.write(&final_header) {
        Ok(_) => {},
        Err(_) => {
            println!("Failed to write header to socket!");
            return;
        }
    };
    // stream.write(&header_hex);
    // stream.write(path.as_bytes());
    let chunks = ssize / 1024;
    let remaining = ssize - (chunks * 1024);

    let now = Instant::now();
    let mut buf = vec![0u8; 1024];
    for x in 0..chunks {
        match file.read_exact(&mut buf) {
            Ok(_) => {
                stream.write(&buf).expect("Failed to write");
                if x % 1000 == 0 {
                    let time_since = now.elapsed().as_secs_f64();
                    println!("Sent chunk {} and total {} bytes. {:.2}% complete in {:.2} seconds at {:.2} kB/s", x, x*1024, (x as f64 / chunks as f64) * 100.0, time_since, x as f64 / time_since);
                }
            },
            Err(e) => {
                println!("Error reading data {:?}", e);
                //Try to read into custom vec
            }
        }
    }

    let mut tbuf = vec![0u8; remaining as usize];
    file.read_exact(&mut tbuf).expect("Failed to read final bytes");
    // file.read_to_end(&mut buf);
    stream.write(&tbuf).expect("Failed to write");
    println!("File transfer completed. Sent {:.2} kBytes in {:.2} seconds at {:.2} kBytes/s", ssize as f64 / 1024.0, now.elapsed().as_secs_f64(), (ssize as f64 / 1024.0) / now.elapsed().as_secs_f64());
    // print::run();
    // vars::run();
}
