use journal::{Journal, JournalError};
use log::{info, LevelFilter};
use nusb::transfer::{ControlIn, ControlOut, ControlType, Recipient, TransferError};
use nusb::{list_devices, Device, DeviceInfo, MaybeFuture};
use std::env;
use std::env::Args;
use std::ffi::CStr;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::process::{ExitCode, Termination};
use std::time::Duration;
use LevelFilter::Info;

const GET_PROTOCOL: u8 = 51;
const SEND_STRING: u8 = 52;
const REQUEST_START: u8 = 53;
const TIMEOUT: Duration = Duration::from_secs(1);

fn main() -> Result<(), Error> {
    Journal::init(Info)?;
    info!("ProjectBulli Android Auto USB Accessory Mode trigger is starting (nusb edition)");
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        return Err(Error::Args(args));
    }
    let bus_number = args[1].parse::<u8>()?;
    let device_number = args[2].parse::<u8>()?;
    list_devices().wait()?.filter(|device| {device.device_address() == device_number && device.busnum() == bus_number}).for_each(|device| {
            match probe_device(&device) {
                Ok(version) => {
                    info!("   ok {}:{} version {}", device.bus_id(), device.device_address(), version)
                }
                Err(e) => {
                    info!("error {}:{} {:?}",  device.bus_id(), device.device_address(), e)
                }
        }
    } );
    Ok(())
}

fn probe_device(device: &DeviceInfo) -> Result<u16, Error> {
    let handle = device.open().wait()?;
    let buffer: [u8; 2] = [0, 2];

  let b = get_protocol(&handle, TIMEOUT).wait()?;
    let version = as_version(b);
    if version < 1 {
        return Err(Error::UnsupportedVersion(version));
    }
    send_string(&handle, 0, c"Android", TIMEOUT).wait()?;
    send_string(&handle, 1, c"Android Auto", TIMEOUT).wait()?;
    send_start(&handle,  TIMEOUT).wait()?;
    Ok(version)
}

fn send_string(
    device:&Device,
    index: u16,
    str: &CStr,
    timeout: Duration,
) -> impl MaybeFuture<Output = Result<(), TransferError>> {
    let data = str.to_bytes_with_nul();
    device.control_out(
            ControlOut {
                control_type: ControlType::Vendor,
                recipient: Recipient::Device,
                request: SEND_STRING,
                index,
                value: 0,
                data,
            },
            timeout,
        )
}

fn send_start(device:&Device, timeout: Duration) -> impl MaybeFuture<Output = Result<(), TransferError>> {
    device.control_out(
        ControlOut {
            control_type: ControlType::Vendor,
            recipient: Recipient::Device,
            request: REQUEST_START,
            index: 0,
            value: 0,
            data: &[],
        },
        timeout,
    )
}
fn get_protocol(device:&Device, timeout: Duration) -> impl MaybeFuture<Output = Result<Vec<u8>, TransferError>> {
     device .control_in(
            ControlIn {
                control_type: ControlType::Vendor,
                recipient: Recipient::Device,
                request: GET_PROTOCOL,
                value: 0,
                index: 0,
                length: size_of::<u16>() as u16,
            },
            timeout,
        )
}
fn as_version(data: Vec<u8>) -> u16 {
    u16::from(data[1]) << 8 | u16::from(data[0])
}

enum Error {
    TransferError(TransferError),
    USB(nusb::Error),
    Parse(ParseIntError),
    Args(Vec<String>),
    UnsupportedVersion(u16),
    Journal(JournalError),
}

impl From<TransferError> for Error {
    fn from(value: TransferError) -> Self {
        Error::TransferError(value)
            }
}
impl From<nusb::Error> for Error {
    fn from(value: nusb::Error) -> Self {
        Error::USB(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::Parse(value)
    }
}

impl From<Args> for Error {
    fn from(value: Args) -> Self {
        Error::Args(value.into_iter().map(String::from).collect())
    }
}

impl From<JournalError> for Error {
    fn from(value: JournalError) -> Self {
        Error::Journal(value)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::USB(usb) => { write!(f, "USB error: {:?}", usb) }
            Error::Parse(parse) => { write!(f, "Parse Error: {:?}", parse)}
            Error::Args(args) => { write!(f, "trouble with arguments, need two arguments: bus-number and device-number but got '{:?}'", args) }
            Error::UnsupportedVersion(u) => { write!(f, "Unsupported android auto version found '{}'", u) }
            Error::Journal(j) => { write!(f, "Journal Error: {:?}", j) }
            Error::TransferError(e) => { write!(f, "Transfer Error: {:?}", e) }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl Termination for Error { //FIXME
    fn report(self) -> ExitCode {
        match self {
            Error::USB(e) => ExitCode::from(01), //TODO mor details from usb error
            Error::Parse(_) => ExitCode::from(20),
            Error::Args(_) => ExitCode::from(30),
            Error::UnsupportedVersion(_) => ExitCode::from(40),
            Error::Journal(_) => ExitCode::from(50),
            Error::TransferError(_) => ExitCode::from(60),
        }
    }
}