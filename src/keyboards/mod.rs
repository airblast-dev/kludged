pub mod rk68;
pub mod steps;

use crate::errors::DeviceNotFound;

use super::errors::GetDeviceError;
use rk68::Rk68;

use hidapi::{DeviceInfo, HidApi, HidResult};
use palette::Srgb;

#[derive(Clone, Debug)]
pub enum KeyboardModels {
    Rk68(Rk68),
}

impl KeyboardModels {
    /// Get all supported, and found keyboards.
    ///
    /// If a single device is being searched for, use [`Keyboard::new`] for that keyboard instead.
    pub fn keyboards() -> HidResult<Vec<Self>> {
        let hid_api = HidApi::new()?;

        KeyboardModels::keyboards_from_hidapi(&hid_api)
    }

    /// Same as [`KeyboardModels::keyboards`], but allows using an already initialized
    /// [`hidapi::HidApi`] instance.
    pub fn keyboards_from_hidapi(hid_api: &HidApi) -> HidResult<Vec<Self>> {
        let mut devices: Vec<_> = hid_api.device_list().collect();

        let mut keyboards: Vec<Self> = vec![];

        {
            while let Some(keyboard) = Rk68::get_from_devices(&mut devices) {
                keyboards.push(keyboard.into());
            }
        }

        Ok(keyboards)
    }
}

pub trait Specs {
    const VID: u16;
    const PID: u16;

    #[inline(always)]
    fn vid() -> u16 {
        Self::VID
    }

    #[inline(always)]
    fn pid() -> u16 {
        Self::PID
    }
}

pub trait Keyboard: Sized + Specs {
    fn new() -> Result<Self, GetDeviceError> {
        let mut hid_api = HidApi::new().map_err(GetDeviceError::from)?;

        hid_api
            .add_devices(Self::VID, Self::PID)
            .map_err(GetDeviceError::from)?;

        let kb = Self::get_from_devices(&mut hid_api.device_list().collect());

        match kb {
            Some(kb) => Ok(kb),
            None => Err(DeviceNotFound.into()),
        }
    }

    /// The correct [`DeviceInfo`]\('s) should be removed and used to construct the instance.
    ///
    /// Any other device information should remain untouched. Takes in a mutable [`Vec`] as in some
    /// cases we may want to store multiple device information.
    ///
    /// When implementing the method the device information is to be checked to see if it matches
    /// the information defined for the device. Information that should generally be checked are,
    /// vendor ID, product ID, usage, usage page, and path.
    ///
    /// ### Note:
    /// Checking the path is not really necessary, but better to do
    /// in case of multiple devices with the same vendor ID, product ID... are found. Mainly
    /// important in case multiple of the same device is plugged in.
    fn get_from_devices(devices: &mut Vec<&DeviceInfo>) -> Option<Self>;
}

pub trait KeyboardColorable: Keyboard {
    const COLOR_ENDPOINT: i32;
    const USAGE: u16;
    const USAGE_PAGE: u16;

    fn set_color<C: Into<Srgb<u8>>>(self, color: C) -> Self;
    fn apply_color(self) -> HidResult<Self>;
}

pub trait KeyboardColorOption {
    type Options;
    fn set_color_parameters<T: Into<Self::Options>>(self, options: T) -> Self;
}

pub trait KeyboardAnimatable {
    const ANIMATION_ENDPOINT: i32;
    const USAGE: u16;
    const USAGE_PAGE: u16;
    type Animation;
    fn set_animation(self, animation: Self::Animation) -> Self;
    fn apply_animation(self) -> HidResult<()>;
}

pub trait KeyboardAnimationOption {
    type Options;
    fn set_animation_parameters<T: Into<Self::Options>>(self, options: T) -> Self;
}

impl From<Rk68> for KeyboardModels {
    fn from(value: Rk68) -> Self {
        Self::Rk68(value)
    }
}
