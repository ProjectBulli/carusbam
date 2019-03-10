extern crate libusb;

use std::time::Duration;
use libusb::{Device, DeviceHandle, Direction, Error, Recipient, request_type, RequestType};

// OAP Control requests:
const ACC_REQ_GET_PROTOCOL :u8 = 51;
const ACC_REQ_SEND_STRING :u8 = 52;
const ACC_REQ_START :u8 = 53;

fn main() {
    match libusb::Context::new() {
        Ok(context) => {
            match context.devices() {
                Ok(devices) => {
                    for device in devices.iter() {
                        prope_device(device)
                    }
                }
                Err(e) => {println!("Error {}", e)}
            }
        }
        Err(e) => {println!("Error {}", e)}
    }
}

fn prope_device(device: Device) {
    match device.open() {
        Ok(handle) => {
            match device.device_descriptor() {
                Ok(device_desc) => {
                    println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
                             device.bus_number(),
                             device.address(),
                             device_desc.vendor_id(),
                             device_desc.product_id());

                    println!("{:?}", read(handle));

                }
                Err(e) => {println!("Error {}", e)}
            }
        }
        Err(_e) => {}
    }
}

fn read(handle: DeviceHandle) -> Result<usize, Error> {
    let mut buffer: [u8;2] = [0,2];
    let read_request = request_type(Direction::In, RequestType::Vendor, Recipient::Device);
    let write_request = request_type(Direction::Out, RequestType::Vendor, Recipient::Device);
    let timeout = Duration::from_secs(1);

    return handle.read_control(read_request, ACC_REQ_GET_PROTOCOL, 0, 0, &mut buffer, timeout)
        .and_then(|_| {
            let version = buffer.as_version();
            println!("Version '{}'", version);
            if version < 1 {
                Result::Err(Error::NoDevice)
            } else  {
                Result::Ok(0)
            }
        })
        .and_then(|_| handle.write_control(write_request, ACC_REQ_SEND_STRING, 0, 0, "Android".as_ref(), timeout))
        .and_then(|_| handle.write_control(write_request, ACC_REQ_SEND_STRING, 0, 1, "Android Auto".as_ref(), timeout))
        .and_then(|_| handle.write_control(write_request, ACC_REQ_START, 0, 0, &mut [], timeout));
}

trait VersionPackage {
    fn as_version(&self) -> u16;
}

impl VersionPackage for [u8;2] {
    fn as_version(&self) -> u16 {
        (self[1] as u16) << 8 | (self[0] as u16)
    }
}
