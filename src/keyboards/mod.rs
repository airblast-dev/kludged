pub mod rk68;
pub mod steps;

use self::rk68::Rk68;

use enum_dispatch::enum_dispatch;
use hidapi::{HidApi, HidResult};
use palette::Srgb;

#[enum_dispatch(Keyboard, Specs)]
#[derive(Clone, Debug)]
pub enum KeyboardModels {
    Rk68(Rk68),
}

impl KeyboardModels {
    /// Get all supported, and found keyboards.
    pub fn keyboards() -> HidResult<Vec<Self>> {
        let hid_api = HidApi::new()?;

        KeyboardModels::keyboards_from_hidapi(&hid_api)
    }

    /// Same as [`KeyboardModels::keyboards`], but allows using an already initialized
    /// [`hidapi::HidApi`] instance.
    ///
    /// This isn't made public as it would require the same library version in order to function.
    /// In other words: It is error prone on the users end.
    pub(crate) fn keyboards_from_hidapi(hid_api: &HidApi) -> HidResult<Vec<Self>> {
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

#[enum_dispatch]
pub trait Keyboard {}

pub trait KeyboardColorable {
    const COLOR_ENDPOINT: i32;
    const USAGE: u16;
    const USAGE_PAGE: u16;

    fn set_color<C: Into<Srgb<u8>>>(&mut self, color: C);
    fn apply_color(&self) -> HidResult<()>;
}

pub trait KeyboardColorOption {
    type Options;
    fn set_color_parameters<T: Into<Self::Options>>(&mut self, options: T);
}

pub trait KeyboardAnimatable {
    const ANIMATION_ENDPOINT: i32;
    const USAGE: u16;
    const USAGE_PAGE: u16;
    type Animation;
    fn set_animation(&mut self, animation: Self::Animation);
    fn apply_animation(&mut self) -> HidResult<()>;
}

pub trait KeyboardAnimationOption {
    type Options;
    fn set_animation_parameters<T: Into<Self::Options>>(&mut self, options: T);
}
