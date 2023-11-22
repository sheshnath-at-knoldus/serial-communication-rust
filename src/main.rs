// use serialport;
// use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
// use std::fs::File;
// use std::io::{Read, Write};
// use std::time::Duration;
//
// // Function to list available ports
// fn list_available_ports() {
//     let ports = serialport::available_ports().expect("No ports found!");
//
//     for p in ports {
//         println!("{}", p.port_name);
//     }
// }
//
// // Function to configure and open a serial port
// fn configure_and_open_serial_port(port_name: &str, baud_rate: u32) -> Box<dyn SerialPort> {
//     serialport::new(port_name, baud_rate)
//         .data_bits(DataBits::Eight)
//         .stop_bits(StopBits::One)
//         .parity(Parity::None)
//         .flow_control(FlowControl::None)
//         .timeout(Duration::from_millis(10))
//         .open()
//         .expect("Failed to open port")
// }
//
// // Function to receive and process data from the serial port
// fn receive_data(port: &mut Box<dyn SerialPort>, output_file_path: &str) {
//     let mut buffer: Vec<u8> = vec![0; 32];
//
//     let mut output_file =
//         File::create(output_file_path).expect("Unable to create a file");
//
//     loop {
//         match port.read(&mut buffer) {
//             Ok(bytes_read) => {
//                 if bytes_read > 0 {
//                     let received_data = &buffer[..bytes_read];
//                     if let Ok(utf_8_data) = std::str::from_utf8(received_data) {
//                         output_file
//                             .write_all(&received_data)
//                             .expect("Unable to write data");
//                         println!("Received: {:?}", utf_8_data);
//
//                         if utf_8_data == "\n" {
//                             break;
//                         }
//
//                         port.clear(serialport::ClearBuffer::Input)
//                             .expect("Unable to clear buffer");
//
//                         std::thread::sleep(Duration::from_millis(50)); // Adjust sleep duration as needed
//                     } else {
//                         println!("Received non-UTF-8 data{:?}", &received_data);
//                     }
//                 }
//             }
//
//             Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
//                 // eprintln!("Timeout occurred. Waiting for more data...");
//             }
//             Err(e) => {
//                 eprintln!("Error reading from serial port: {}", e);
//                 break; // Break out of the loop on error
//             }
//         }
//     }
// }
//
// fn main() {
//     // List available ports
//     list_available_ports();
//
//     // Configure and open serial port
//     let mut port = configure_and_open_serial_port("/dev/ttyUSB0", 9600);
//
//     // Receive data from the serial port
//     receive_data(&mut port, "resources/received.txt");
// }
//
//

















use serialport;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;

fn main() {

    let ports = serialport::available_ports().expect("No ports found!");

    for p in ports {
        println!("{}", p.port_name);
    }

    let mut port = serialport::new("/dev/ttyUSB0", 9600)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .parity(Parity::None)
        .flow_control(FlowControl::None)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");


    // Receiver code snippet
    let mut buffer: Vec<u8> = vec![0;2028];

    let mut output_file =
        File::create("resources/received.txt").expect("unable to create a file");

    port.clear(serialport::ClearBuffer::All).expect("unable to clear buffer");
    // Read in a loop1..

    loop{
        match port.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read > 0 {
                    let received_data = &buffer[..bytes_read];
                    if std::str::from_utf8(received_data).is_ok() {
                        let utf_8_data = String::from_utf8_lossy(received_data);
                        output_file
                            .write_all(&received_data)
                            .expect("unable to write data");
                        println!("Received: {:?}", utf_8_data);

                        if utf_8_data=="\n" {
                            drop(output_file);
                            std::thread::sleep(Duration::from_millis(50)); // Adjust sleep duration as needed

                            send_acknowledgement(port.try_clone().expect("unable to clone "));
                             break;
                        }

                        port.clear(serialport::ClearBuffer::Input).expect("unable to clear buffer");

                        std::thread::sleep(Duration::from_millis(50)); // Adjust sleep duration as needed
                    } else {
                        println!("Received non-UTF-8 data{:?}",&received_data);
                    }

                }
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                eprintln!("Timeout occurred. Waiting for more data...");
            }
            Err(e) => {
                eprintln!("Error reading from serial port: {}", e);
                 break; // Break out of the loop on error
            }
        }
    }

}


fn send_acknowledgement(mut port: Box<dyn SerialPort>){
    let mut file = File::open("resources/857.txt").expect("unable to open 894 acknowledgement file");
    let mut file_buffer  :Vec<u8> = Vec::new();
    file.read_to_end(&mut file_buffer ).expect("unable to write ");

    let extra_buffer =file_buffer.clone();
    let size_of_buffer= extra_buffer.len();

    for mut data in extra_buffer.chunks(size_of_buffer/8) {
        port.clear(serialport::ClearBuffer::All).expect("unable to clear buffer");
        port.write(&mut data).expect("write failed");
        port.flush().expect("unable to flush");
        std::thread::sleep(Duration::from_millis(500));
        println!("acknowledment send ->{:?}",data);
    }

}




