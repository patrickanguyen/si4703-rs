/// Seek a channel, listen for 5 seconds and then seek again.
use embedded_hal::delay::DelayNs;
use linux_embedded_hal::{
    gpio_cdev::{self, LineRequestFlags},
    CdevPin, Delay, I2cdev,
};
use nb::block;
use si4703::{
    fill_with_rds_radio_text, reset_and_select_i2c_method1, ChannelSpacing, DeEmphasis, Si4703,
    TuneChannel, Volume,
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

    radio.enable_auto_gain_control().unwrap();
    radio.enable_rds(si4703::RdsMode::Standard).unwrap();
    radio.set_volume(Volume::Dbfsm28).unwrap();
    radio.set_deemphasis(DeEmphasis::Us75).unwrap();
    radio.set_channel_spacing(ChannelSpacing::Khz200).unwrap();
    radio.unmute().unwrap();

    // use STC interrupt pin method
    let stc_line = chip.get_line(27).unwrap();
    let stc_line_handle = stc_line
        .request(LineRequestFlags::INPUT, 0, "si4703")
        .unwrap();
    let mut stc_int = CdevPin::new(stc_line_handle).unwrap();

    let channel = std::env::var("STATION").unwrap().parse().unwrap();
    block!(radio.tune_with_stc_int_pin(TuneChannel::Mhz(channel), &mut stc_int)).unwrap();

    println!("BlockA, BlockB, BlockC, BlockD");

    loop {
        if radio.rds_ready().unwrap() {
            let data = radio.rds_data().unwrap();
            println!(
                "{:#04X}, {:#04X}, {:#04X}, {:#04X}",
                data.a.data, data.b.data, data.c.data, data.d.data,
            );
            let mut text = [' '; 64];
            let is_clear = fill_with_rds_radio_text(&mut text, &data);
            if is_clear {
                text = ['\0'; 64];
            }
            let s: String = text.iter().collect();
            println!("Clear: {is_clear:?} RT: {s}");
        }
        delay.delay_ms(100);
    }
}
