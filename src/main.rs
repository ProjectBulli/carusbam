extern crate libusb;

use std::time::Duration;
use libusb::{Device, DeviceHandle, Direction, Error, Recipient, request_type, RequestType};

const GET_PROTOCOL:u8 = 51;
const SEND_STRING:u8 = 52;
const REQUEST_START:u8 = 53;

fn main() {
    match libusb::Context::new() {
        Ok(context) => {
            match context.devices() {
                Ok(devices) => {
                    for device in devices.iter() {
                        probe_device(device)
                    }
                }
                Err(e) => {println!("Error {}", e)}
            }
        }
        Err(e) => {println!("Error {}", e)}
    }
}

fn probe_device(device: Device) {
    match device.open() {
        Ok(handle) => {
            println!("{:?}", probe(handle));
        }
        Err(e) => {println!("Error {}", e)}
    }
}

fn probe(handle: DeviceHandle) -> Result<usize, Error> {
    let mut buffer: [u8;2] = [0,2];
    let read_request = request_type(Direction::In, RequestType::Vendor, Recipient::Device);
    let write_request = request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
    let timeout = Duration::from_secs(1);

    return handle.read_control(read_request, GET_PROTOCOL, 0, 0, &mut buffer, timeout)
        .and_then(|_| {
            let version = buffer.as_version();
            println!("Version '{}'", version);
            if version < 1 {
                Result::Err(Error::NoDevice)
            } else  {
                Result::Ok(0)
            }
        })
        .and_then(|_| handle.write_control(write_request, SEND_STRING, 0, 0, "Android".as_ref(), timeout))
        .and_then(|_| handle.write_control(write_request, SEND_STRING, 0, 1, "Android Auto".as_ref(), timeout))
        .and_then(|_| handle.write_control(write_request, REQUEST_START, 0, 0, &mut [], timeout));
}

trait VersionPackage {
    fn as_version(&self) -> u16;
}

impl VersionPackage for [u8;2] {
    fn as_version(&self) -> u16 {
        (self[1] as u16) << 8 | (self[0] as u16)
    }
}
