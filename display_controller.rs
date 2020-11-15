use serialport::SerialPort;
use serialport::SerialPortSettings;
use serialport::prelude::*;
use std::time::Duration;

struct DisplayController {
    brightness: u32,
    backlight: bool,
    serial_settings: serialport::SerialPortSettings,
    serial_port: Box<dyn SerialPort>
}

impl DisplayController {
    fn set_brightness(&self, brightness: u32) {
        brightness = brightness - 100;
        let scaled_brightness = (brightness - 0) * ((950 - 10)/(100 - 0)) + 10;
        let command = format!("set brightness={scaled_brightness}", scaled_brightness=scaled_brightness);
        self.serial_port.write(command.as_bytes());
    }
}

fn controller_generator(serial_port: &str) -> &DisplayController {
    let serial_settings = SerialPortSettings {
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1),
    };
    let controller = &DisplayController{
        brightness: 100,
        backlight: true,
        serial_settings: serial_settings,
        serial_port: Box::new( serialport::open_with_settings("/dev/TTYAMA0", serial_settings) ) 
    };
    return controller
}

fn main() {

}