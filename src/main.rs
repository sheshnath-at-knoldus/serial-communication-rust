/*


use std::thread;
use std::io::Write;
use std::time::Duration;

use serialport::Parity;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    //let mut port = serialport::new("/dev/ttyS0", 9600)
    //    .timeout(Duration::from_millis(10000))
    //    .open()
    //    .expect("Failed to open port");

    let mut port = serialport::new("/dev/ttyS0",16550)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .parity(Parity::None)
        .timeout(Duration::from_millis(1000))
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open {}", e);
            ::std::process::exit(1);
        });

    let output = "Hello".as_bytes();
    port.write_all(output).expect("Write failed!");

    //let mut write_buffer:Vec<u8> = vec![0;10];
    //write_buffer[0]= b'H';
    // write_buffer[1]= b'e';
    // port.write_all(&write_buffer[..2]).expect("write failed");

    println!("data send -> {:?}", output);



    thread::sleep(Duration::from_millis(10000));


    drop(port);
}

*/

use serialport::SerialPort;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

fn main() {
    let mut file = File::open("resources/857.txt").expect("unable to open file");
    let mut file_buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut file_buffer).expect("unable to write");
    let mut extra_buffer = file_buffer.clone();

    let size_of_buffer = file_buffer.len();

    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/ttyS0", 9600)
        .data_bits(serialport::DataBits::Eight)
        .stop_bits(serialport::StopBits::One)
        .parity(Parity::None)
        .flow_control(FlowControl::None)
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open {}", e);
            ::std::process::exit(1);
        });

    println!("size of buffer{}", size_of_buffer);

    for mut data in extra_buffer.chunks(size_of_buffer / 8) {
        port.clear(serialport::ClearBuffer::All)
            .expect("unable to clear buffer");
        let mut _output = "HELLO".as_bytes();
        port.write(&mut data).expect("write failed");
        port.flush().expect("unable to clear buffer");
        std::thread::sleep(Duration::from_millis(500));

        println!("write data -> {:?}", data);

        if data == &[10] {
                    std::thread::sleep(Duration::from_millis(500));

            println!("data{:?}", data);
            receive_acknowledgement(port.try_clone().expect("unable to clone "));
            break;
        }
    }

   
}

//function to read acknowledgement

fn receive_acknowledgement(mut port: Box<dyn SerialPort>) {


    let mut buffer: Vec<u8> = vec![0;1024];
    let mut output_file = File::create("resources/acknowledgement.txt")
        .expect("unable to create acknowledgement file");
     println!("inside acknowledgement function2");
    
    port.clear(serialport::ClearBuffer::Input).expect("unable to clear buffer");

    loop {
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let received_data = &buffer[..bytes_read];
                      if std::str::from_utf8(received_data).is_ok() {
                        let utf_8_data = String::from_utf8_lossy(received_data);

                        output_file
                            .write_all(&received_data)
                            .expect("unable to write data in file");

                        println!("acknowledgement -> {:?}", utf_8_data);

                      //  if received_data == &[10] {
                        //    break;
                       // }
                        port.clear(serialport::ClearBuffer::Input)
                            .expect(" unable to clear buffer");

                        std::thread::sleep(Duration::from_millis(50));
                    } else {
                        println!("Received non-utf data {:?}", &received_data);
                    }
                }
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                eprintln!("time out occured . Waiting for more data ....");
            }
            Err(e) => {
                eprintln!("Error reading from serial port : {}", e);
                break;
            }
        }
    }
}
