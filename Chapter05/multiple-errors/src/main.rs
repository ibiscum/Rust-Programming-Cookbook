use std::fmt;
use std::io;
use std::error::Error;

#[derive(Debug)]
pub struct InvalidDeviceIdError(usize);
#[derive(Debug)]
pub struct DeviceNotPresentError(usize);
#[derive(Debug)]
pub struct UnexpectedDeviceStateError {}

#[derive(Debug)]
pub enum ErrorWrapper {
    Io(io::Error),
    Db(InvalidDeviceIdError),
    Device(DeviceNotPresentError), 
    Agent(UnexpectedDeviceStateError)
}

impl Error for ErrorWrapper {}

impl fmt::Display for ErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorWrapper::Io(ref e) => write!(f, "{}", e), 
            ErrorWrapper::Db(ref e) => write!(f, "Device with id \"{}\" not found", e.0),
            ErrorWrapper::Device(ref e) => write!(f, "Device with id \"{}\" is currently unavailable", e.0),
            ErrorWrapper::Agent(_) => write!(f, "Unexpected device state")
        }
    }
}


fn main() {
    println!("{}", ErrorWrapper::Io(io::Error::from(io::ErrorKind::InvalidData)));
    println!("{}", ErrorWrapper::Db(InvalidDeviceIdError(42)));
    println!("{}", ErrorWrapper::Device(DeviceNotPresentError(42)));
    println!("{}", ErrorWrapper::Agent(UnexpectedDeviceStateError {}));
}