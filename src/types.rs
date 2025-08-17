use core::marker::PhantomData;

/// Errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus communication error
    I2C(E),
    /// Invalid input data provided
    InvalidInputData,
    /// Seek operation failed / Band limit reached
    SeekFailed,
}

/// Errors for operations involving I2C communication as well
/// as interaction with pins
#[derive(Debug)]
pub enum ErrorWithPin<CommE, PinE> {
    /// I²C bus communication error
    I2C(CommE),
    /// Error while communicating with pin
    Pin(PinE),
    /// Invalid input data provided
    InvalidInputData,
    /// Seek operation failed / Band limit reached
    SeekFailed,
}

impl<CommE, PinE> From<Error<CommE>> for ErrorWithPin<CommE, PinE> {
    fn from(error: Error<CommE>) -> Self {
        match error {
            Error::I2C(e) => ErrorWithPin::I2C(e),
            Error::InvalidInputData => ErrorWithPin::InvalidInputData,
            Error::SeekFailed => ErrorWithPin::SeekFailed,
        }
    }
}

/// IC markers
#[doc(hidden)]
pub mod ic {
    /// Used for Si4702 devices
    pub struct Si4702(());
    /// Used for Si4703 devices
    pub struct Si4703(());
}

/// markers
#[doc(hidden)]
pub mod marker {
    use super::super::private;
    pub trait WithRds: private::Sealed {}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OperationState {
    Idle,
    Busy,
    WaitingForStcToClear(bool),
}

/// Si4703 device driver
#[derive(Debug)]
pub struct Si4703<I2C, IC> {
    pub(crate) i2c: I2C,
    pub(crate) seeking_state: OperationState,
    pub(crate) tuning_state: OperationState,
    pub(crate) _ic: PhantomData<IC>,
}

/// Seek mode
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SeekMode {
    /// Wrap at the end of the band (default)
    #[default]
    Wrap,
    /// Stop at the end of the band
    NoWrap,
}

/// Seek direction
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SeekDirection {
    /// Down (default)
    #[default]
    Down,
    /// Up
    Up,
}

/// De-emphasis
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DeEmphasis {
    /// 75 us (used in USA) (default)
    #[default]
    Us75,
    /// 50 us (used in Europe, Australia and Japan)
    Us50,
}

/// GPIO1 configuration
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Gpio1Config {
    /// High impedance (default)
    #[default]
    HighImpedance,
    /// High
    High,
    /// Low
    Low,
}

/// GPIO2 configuration
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Gpio2Config {
    /// High impedance (default)
    #[default]
    HighImpedance,
    /// STC/RDS interrupt (logic high until interrupt occurs)
    StcRdsInterrupt,
    /// High
    High,
    /// Low
    Low,
}

/// GPIO3 configuration
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Gpio3Config {
    /// High impedance (default)
    #[default]
    HighImpedance,
    /// Mono/Stereo indicator (logic low for mono, high for stereo)
    MonoStereoIndicator,
    /// High
    High,
    /// Low
    Low,
}

/// RDS mode
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum RdsMode {
    /// Standard (default)
    #[default]
    Standard,
    /// Verbose
    Verbose,
}

/// Band
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Band {
    /// 87.5-108 Mhz (USA, Europe) (default)
    #[default]
    Mhz875_108,
    /// 76 - 108 MHz (Japan wide band)
    Mhz76_108,
    /// 76 - 90 MHz (Japan)
    Mhz76_90,
}

/// Channel spacing
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ChannelSpacing {
    /// 200 kHz (USA, Australia) (default)
    #[default]
    Khz200,
    /// 100 kHz (Europe, Japan)
    Khz100,
    /// 50 kHz
    Khz50,
}

/// Output mode
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum OutputMode {
    /// Stereo (default)
    #[default]
    Stereo,
    /// Mono
    Mono,
}

/// Stereo to mono blend level
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum StereoToMonoBlendLevel {
    /// 19–37 RSSI dBμV (–12 dB)
    Dbuv19_37,
    /// 25–43 RSSI dBμV (–6 dB).
    Dbuv25_43,
    /// 31–49 RSSI dBμV (default)
    #[default]
    Dbuv31_49,
    /// 37–55 RSSI dBμV (+6 dB)
    Dbuv37_55,
}

/// Volume
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Volume {
    /// Mute (0 volume) (default)
    #[default]
    Mute,
    /// –58 dBFS (extended volume range).
    Dbfsm58,
    /// –56 dBFS (extended volume range).
    Dbfsm56,
    /// –54 dBFS (extended volume range).
    Dbfsm54,
    /// –52 dBFS (extended volume range).
    Dbfsm52,
    /// –50 dBFS (extended volume range).
    Dbfsm50,
    /// –48 dBFS (extended volume range).
    Dbfsm48,
    /// –46 dBFS (extended volume range).
    Dbfsm46,
    /// –44 dBFS (extended volume range).
    Dbfsm44,
    /// –42 dBFS (extended volume range).
    Dbfsm42,
    /// –40 dBFS (extended volume range).
    Dbfsm40,
    /// –38 dBFS (extended volume range).
    Dbfsm38,
    /// –36 dBFS (extended volume range).
    Dbfsm36,
    /// –34 dBFS (extended volume range).
    Dbfsm34,
    /// –32 dBFS (extended volume range).
    Dbfsm32,
    /// –30 dBFS (extended volume range).
    Dbfsm30,
    /// –28 dBFS.
    Dbfsm28,
    /// –26 dBFS.
    Dbfsm26,
    /// –24 dBFS.
    Dbfsm24,
    /// –22 dBFS.
    Dbfsm22,
    /// –20 dBFS.
    Dbfsm20,
    /// –18 dBFS.
    Dbfsm18,
    /// –16 dBFS.
    Dbfsm16,
    /// –14 dBFS.
    Dbfsm14,
    /// –12 dBFS.
    Dbfsm12,
    /// –10 dBFS.
    Dbfsm10,
    /// –8 dBFS.
    Dbfsm8,
    /// –6 dBFS.
    Dbfsm6,
    /// –4 dBFS.
    Dbfsm4,
    /// –2 dBFS.
    Dbfsm2,
    /// 0 dBFS (maximum).
    Dbfs0,
}

/// Softmute Attack/Recover Rate
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SoftmuteRate {
    /// Fastest (default)
    #[default]
    Fastest,
    /// Fast
    Fast,
    /// Slow
    Slow,
    /// Slowest
    Slowest,
}

/// Softmute Attenuation
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SoftmuteAttenuation {
    /// 16 dB (default)
    #[default]
    Db16,
    /// 14 dB
    Db14,
    /// 12 dB
    Db12,
    /// 10 dB
    Db10,
}

/// Required channel SNR for a valid seek.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SeekSnrThreshold {
    /// Disabled (default)
    #[default]
    Disabled,
    /// Enabled
    ///
    /// The value provided corresponds to the stops:
    /// - Minimum (most stops): `Enable(1)`
    /// - Maximum (fewest stops): `Enable(7)`
    Enabled(u8),
}

/// Allowable number of FM impulses for a valid seek channel.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SeekFmImpulseThreshold {
    /// Disabled (default)
    #[default]
    Disabled,
    /// Enabled
    ///
    /// The value provided corresponds to the stops:
    /// - Maximum (most stops): `Enable(1)`
    /// - Minimum (fewest stops): `Enable(15)`
    Enabled(u8),
}

/// Tune channel frequency
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TuneChannel {
    /// Raw value for the channel select (10 bits)
    Raw(u16),
    /// Target frequency in MHz. The raw value will be aproximated taking
    /// the configured band and spacing into account
    Mhz(f32),
}

/// RDS block errors
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum RdsBlockErrors {
    /// No errors
    #[default]
    None,
    /// 1-2 errors requiring correction.
    OneOrTwo,
    /// 3-5 errors requiring correction.
    ThreeToFive,
    /// 6+ errors or error in checkword. Correction not possible.
    TooMany,
}

/// RDS block data
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct RdsBlockData {
    /// Data
    pub data: u16,
    /// Block errors
    pub errors: RdsBlockErrors,
}

/// RDS data data
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct RdsData {
    /// Block A
    pub a: RdsBlockData,
    /// Block B
    pub b: RdsBlockData,
    /// Block C
    pub c: RdsBlockData,
    /// Block D
    pub d: RdsBlockData,
}

/// RDS radio text data
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RdsRadioTextData {
    /// Two characters
    Two(char, char),
    /// Four characters
    Four(char, char, char, char),
}
/// RDS radio text
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RdsRadioText {
    /// Screen clear requested (Text A/B)
    pub screen_clear: bool,
    /// Text + segment offset [0-63]
    pub text: Option<(RdsRadioTextData, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! default_test {
        ($name:ident, $type:ident, $default:ident) => {
            #[test]
            fn $name() {
                assert_eq!($type::$default, $type::default());
            }
        };
    }

    default_test!(default_seek_mode, SeekMode, Wrap);
    default_test!(default_seek_direction, SeekDirection, Down);
    default_test!(default_de, DeEmphasis, Us75);
    default_test!(default_gpio1, Gpio1Config, HighImpedance);
    default_test!(default_gpio2, Gpio2Config, HighImpedance);
    default_test!(default_gpio3, Gpio3Config, HighImpedance);
    default_test!(default_rds_mode, RdsMode, Standard);
    default_test!(default_band, Band, Mhz875_108);
    default_test!(default_spacing, ChannelSpacing, Khz200);
    default_test!(default_output_mode, OutputMode, Stereo);
    default_test!(default_volume, Volume, Mute);
    default_test!(default_softmute_att, SoftmuteAttenuation, Db16);
    default_test!(default_softmute_rate, SoftmuteRate, Fastest);
    default_test!(default_blend, StereoToMonoBlendLevel, Dbuv31_49);
    default_test!(default_snr, SeekSnrThreshold, Disabled);
    default_test!(default_fm_impulse, SeekFmImpulseThreshold, Disabled);
    default_test!(default_rds_block_err, RdsBlockErrors, None);
}
