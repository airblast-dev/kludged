use std::fmt::Display;

use hidapi::HidError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetDeviceError {
    #[error(transparent)]
    HidApiError(#[from] HidError),
    #[error(transparent)]
    DeviceNotFound(#[from] DeviceNotFound),
}

#[derive(Clone, Copy, Debug, Error)]
pub struct DeviceNotFound;

impl Display for DeviceNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The requested device could not be found.")
    }
}
