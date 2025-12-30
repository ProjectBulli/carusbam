use journal::{Journal, JournalError};
use log::{info, LevelFilter};
use rusb::{devices, request_type, Device, Direction, Recipient, RequestType, UsbContext};
use std::env;
use std::env::Args;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::process::{ExitCode, Termination};
use std::time::Duration;
use LevelFilter::Info;

const GET_PROTOCOL: u8 = 51;
const SEND_STRING: u8 = 52;
const REQUEST_START: u8 = 53;
const TIMEOUT: Duration = Duration::from_secs(1);

const READ_REQUEST:u8 = request_type(Direction::In, RequestType::Vendor, Recipient::Device);
const WRITE_REQUEST:u8 = request_type(Direction::Out, RequestType::Vendor, Recipient::Device);

fn main() -> Result<(), Error> {
    Journal::init(Info)?;
    info!("ProjectBulli \"Android Auto USB Accessory Mode\" is starting");
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        return Err(Error::Args(args));
    }
    let bus_number = args[1].parse::<u8>()?;
    let device_number = args[2].parse::<u8>()?;
    for device in devices()?.iter()
        .filter(|device| {device.bus_number() == bus_number && device.address() == device_number}) {
        let bus = device.bus_number();
        let address = device.address();
            match probe_device(device) {
                Ok(version) => {
                    info!("   ok {}:{} version {}", bus, address, version)
                }
                Err(e) => {
                    info!("error {}:{} {:?}", bus, address, e)
                }
        }
    }
    Ok(())
}

fn probe_device<T: UsbContext>(device: Device<T>) -> Result<u16, Error> {
    let handle = device.open()?;
    let mut buffer: [u8; 2] = [0, 2];

    handle.read_control(READ_REQUEST, GET_PROTOCOL, 0, 0, &mut buffer, TIMEOUT)?;
    let version = as_version(buffer);
    if version < 1 {
        return Err(Error::UnsupportedVersion(version));
    }
    handle.write_control(WRITE_REQUEST, SEND_STRING, 0, 0, "Android".as_ref(), TIMEOUT)?;
    handle.write_control(WRITE_REQUEST, SEND_STRING, 0, 1, "Android Auto".as_ref(), TIMEOUT)?;
    handle.write_control(WRITE_REQUEST, REQUEST_START, 0, 0, &mut [], TIMEOUT)?;
    Ok(version)
}

fn as_version(data: [u8; 2]) -> u16 {
    u16::from(data[1]) << 8 | u16::from(data[0])
}

enum Error {
    USB(rusb::Error),
    Parse(ParseIntError),
    Args(Vec<String>),
    UnsupportedVersion(u16),
    Journal(JournalError),
}

impl From<rusb::Error> for Error {
    fn from(value: rusb::Error) -> Self {
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
            Error::USB(e) => {
                match e {
                    rusb::Error::Io            => ExitCode::from(01),
                    rusb::Error::InvalidParam  => ExitCode::from(02),
                    rusb::Error::Access        => ExitCode::from(03),
                    rusb::Error::NoDevice      => ExitCode::from(04),
                    rusb::Error::NotFound      => ExitCode::from(05),
                    rusb::Error::Busy          => ExitCode::from(06),
                    rusb::Error::Timeout       => ExitCode::from(07),
                    rusb::Error::Overflow      => ExitCode::from(08),
                    rusb::Error::Pipe          => ExitCode::from(09),
                    rusb::Error::Interrupted   => ExitCode::from(10),
                    rusb::Error::NoMem         => ExitCode::from(11),
                    rusb::Error::NotSupported  => ExitCode::from(12),
                    rusb::Error::Other         => ExitCode::from(13),
                    rusb::Error::BadDescriptor => ExitCode::from(14),
                }
            },
            Error::Parse(e) => ExitCode::from(20),
            Error::Args(_) => ExitCode::from(30),
            Error::UnsupportedVersion(_) => ExitCode::from(40),
            Error::Journal(_) => ExitCode::from(50),
        }
    }
}