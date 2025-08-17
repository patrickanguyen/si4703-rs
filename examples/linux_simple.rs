use embedded_hal::delay::DelayNs;
use linux_embedded_hal::{
    gpio_cdev::{self, LineRequestFlags},
    CdevPin, Delay, I2cdev,
};
use nb::block;
use si4703::{
    reset_and_select_i2c_method1, ChannelSpacing, DeEmphasis, SeekDirection, SeekMode, Si4703,
    Volume,
};

fn main() {
    let mut delay = Delay {};
    let mut chip = gpio_cdev::Chip::new("/dev/gpiochip0").unwrap();
    {
        // Reset and communication protocol selection must be done beforehand
        let sda_line = chip.get_line(0).unwrap();
        let rst_line = chip.get_line(17).unwrap();

        let sda_line_handle = sda_line
            .request(LineRequestFlags::OUTPUT, 1, "si4703")
            .unwrap();
        let rst_line_handle = rst_line
            .request(LineRequestFlags::OUTPUT, 1, "si4703")
            .unwrap();

        let mut sda = CdevPin::new(sda_line_handle).unwrap();
        let mut rst = CdevPin::new(rst_line_handle).unwrap();
        reset_and_select_i2c_method1(&mut rst, &mut sda, &mut delay).unwrap();
    }
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut radio = Si4703::new(dev);
    radio.enable_oscillator().unwrap();
    // Wait for the oscillator to stabilize
    delay.delay_ms(500);
    radio.enable().unwrap();
    // Wait for powerup
    delay.delay_ms(110);

    radio.set_volume(Volume::Dbfsm28).unwrap();
    radio.set_deemphasis(DeEmphasis::Us50).unwrap();
    radio.set_channel_spacing(ChannelSpacing::Khz100).unwrap();
    radio.unmute().unwrap();

    let stc_line = chip.get_line(27).unwrap();
    let stc_line_handle = stc_line
        .request(LineRequestFlags::INPUT, 0, "si4703")
        .unwrap();
    let mut stc_int = CdevPin::new(stc_line_handle).unwrap();
    // Seek using STC interrupt pin
    block!(radio.seek_with_stc_int_pin(SeekMode::Wrap, SeekDirection::Up, &mut stc_int)).unwrap();
    let channel = radio.channel().unwrap_or(-1.0);
    println!("Found channel at {:1} MHz", channel);
}
