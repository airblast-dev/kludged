pub mod keys;

use std::{thread::sleep, time::Duration};

use hidapi::{DeviceInfo, HidApi};
use palette::Srgb;
use strum::IntoEnumIterator;

use self::keys::Keys;

use super::{
    steps::Steps, Keyboard, KeyboardAnimatable, KeyboardAnimationOption, KeyboardColorOption,
    KeyboardColorable, Specs,
};

/// The number of steps needed for a color setting.
const COLOR_STEP_COUNT: usize = 7;

/// Number of bytes we need to send for each step.
const COLOR_LEN: usize = 65;

/// Total number of bytes that will be written for a color setting.
const COLOR_DATA_LEN: usize = COLOR_LEN * COLOR_STEP_COUNT;

/// The endpoint used to send color, and animation data.
const ENDPOINT: i32 = 0x01;

/// Number of steps needed for setting an animation.
const ANIMATION_STEP_COUNT: usize = 1;

/// Number of bytes needed to set an animation.
const ANIMATION_LEN: usize = 65;

#[derive(Clone, Debug)]
pub struct Rk68 {
    device_info: DeviceInfo,
    color_steps: Steps<COLOR_DATA_LEN>,
    animation_steps: Steps<ANIMATION_LEN>,
}

impl Rk68 {
    /// Create an unchecked keyboard via [`DeviceInfo`].
    ///
    /// While it is not recommended you call this directly:
    /// It makes it possible to test if a different model keyboard communicates the same way as this one.
    /// (Such as Rk68-V1 being also compatible with Rk68-V2)
    /// Before calling this function you should at least verify if the keyboard in question, works
    /// in a similar way to this keyboard. This can be done by logging the packets, and comparing them to what is
    /// done for this device.
    ///
    /// # ⚠️ Warning:
    /// The device being written to can be bricked, or bugged when a write is performed, if
    /// unsupported device information is passed.
    pub(crate) fn new(device_info: DeviceInfo) -> Self {
        let color_steps = {
            // 0x0A, and 0x07 are sent on each step. In a sense they serve as an indicator that
            // this is a color packet.
            let mut steps = Steps::new(COLOR_STEP_COUNT, COLOR_LEN, &[0x0A, 0x07]);

            // No idea what these do, but the official application does it (excluding the last
            // value, which is for setting sleep duration).
            steps.data[3..6].copy_from_slice(&[0x03, 0x7E, Sleep::default() as u8]);

            // The 3rd value in the array indicates its step count (in other words: being the N'th step).
            // The first value starts by one, and is incremented for each step.
            steps
                .steps_mut()
                .zip(1..COLOR_STEP_COUNT + 1)
                .for_each(|(step, i)| step[2] = i as u8);

            steps
        };

        let animation_steps = Steps::new(
            ANIMATION_STEP_COUNT,
            ANIMATION_LEN,
            &[
                0x0A,
                0x01,
                0x01,
                0x02,
                0x29,
                0x01,
                0x00,
                Speed::default() as u8,
                Brightness::default() as u8,
                0x00, // Default Colors.
                0x00,
                0x00,
                0x00, // Color Mixing.
                Sleep::Never as u8,
            ],
        );

        Self {
            device_info,
            color_steps,
            animation_steps,
        }
    }

    pub(crate) fn get_from_devices(devices: &mut Vec<&DeviceInfo>) -> Option<Self> {
        let device_info = {
            let color_device_index = devices.iter().position(|inf| {
                inf.vendor_id() == Self::VID
                    && inf.product_id() == Self::PID
                    && inf.interface_number() == Self::COLOR_ENDPOINT
                    && inf.usage() == <Self as KeyboardColorable>::USAGE
                    && inf.usage_page() == <Self as KeyboardColorable>::USAGE_PAGE
            })?;

            devices.remove(color_device_index).clone()
        };

        Some(Self::new(device_info))
    }
}

impl Specs for Rk68 {
    const VID: u16 = 0x0258A;
    const PID: u16 = 0x005E;
}

impl Keyboard for Rk68 {}

impl KeyboardColorable for Rk68 {
    const COLOR_ENDPOINT: i32 = ENDPOINT;
    const USAGE_PAGE: u16 = 1;
    const USAGE: u16 = 128;
    fn set_color<C: Into<Srgb<u8>>>(&mut self, color: C) {
        let color: Srgb<u8> = color.into();

        let colors = [color.red, color.green, color.blue];

        Keys::iter().map(|key| key.indexes()).for_each(|indexes| {
            indexes.into_iter().zip(colors).for_each(|(index, color)| {
                self.color_steps[index] = color;
            })
        })
    }
    fn apply_color(&self) -> hidapi::HidResult<()> {
        let color_device = self.device_info.open_device(&HidApi::new()?)?;

        self.color_steps.steps().try_for_each(|step| {
            let write_result = color_device.send_feature_report(step).map(|_| ());
            sleep(Duration::from_millis(5));

            write_result
        })
    }
}

#[derive(Clone, Debug, Default)]
pub enum Sleep {
    FiveMinutes = 1,
    #[default]
    TenMinutes = 2,
    TwentyMinutes = 3,
    ThirtyMinutes = 4,
    Never,
}

impl From<Sleep> for ColorOptions {
    fn from(value: Sleep) -> Self {
        Self { sleep: value }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ColorOptions {
    pub sleep: Sleep,
}

impl KeyboardColorOption for Rk68 {
    type Options = ColorOptions;
    fn set_color_parameters<T: Into<Self::Options>>(&mut self, options: T) {
        self.color_steps.data[5] = options.into().sleep as u8;
    }
}

pub enum Animation {
    NeonStream = 1,
    RipplesShining = 2,
    RotatingWindmill = 3,
    SineWave = 4,
    RainbowRoulette = 5,
    StarsTwinkle = 6,
    LayerUponLayer = 7,
    RichAndHonored = 8,
    MarqueeEffect = 9,
    RotatingStorm = 10,
    SerpentineHorse = 11,
    RetroSnake = 12,
    DiagonalTransformer = 13,
    Ambilight = 14,
    Streamer = 15,
    Steady = 16,
    Breathing = 17,
    Neon = 18,
    ShadowDisappear = 19,
    FlashAway = 20,
}

impl KeyboardAnimatable for Rk68 {
    const ANIMATION_ENDPOINT: i32 = ENDPOINT;
    const USAGE: u16 = 1;
    const USAGE_PAGE: u16 = 128;
    type Animation = Animation;
    fn set_animation(&mut self, animation: Self::Animation) {
        self.animation_steps.data[7] = animation as u8;
    }

    fn apply_animation(&mut self) -> hidapi::HidResult<()> {
        let device = self.device_info.open_device(&HidApi::new()?)?;

        device.send_feature_report(&self.animation_steps)?;

        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct AnimationOptions {
    pub color_mix: bool,
    pub color: Srgb<u8>,
    pub speed: Speed,
    pub sleep: Sleep,
    pub brightness: Brightness,
}

#[derive(Clone, Debug, Default)]
pub enum Speed {
    #[default]
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(Clone, Debug, Default)]
/// Zero is off, each one after that is 20% of the total brightness.
pub enum Brightness {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    #[default]
    Five = 5,
}

impl KeyboardAnimationOption for Rk68 {
    type Options = AnimationOptions;
    fn set_animation_parameters<T: Into<Self::Options>>(&mut self, options: T) {
        let options: Self::Options = options.into();

        // Set animation speed.
        self.animation_steps.data[7] = options.speed as u8;

        self.animation_steps.data[8] = options.brightness as u8;

        // Set the colors.
        self.animation_steps.data[9..12].copy_from_slice(&[
            options.color.red,
            options.color.green,
            options.color.blue,
        ]);

        // Set color mixing.
        self.animation_steps.data[12] = if options.color_mix { 1 } else { 0 };

        // Set the sleep.
        self.animation_steps.data[13] = options.sleep as u8;
    }
}
