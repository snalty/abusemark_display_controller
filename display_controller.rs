use serialport::SerialPortSettings;
use serialport::prelude::*;
use std::time::Duration;
use std::env;

struct DisplayController {
    brightness: u32,
    backlight: bool,
    serial_settings: serialport::SerialPortSettings,
    serial_port: std::boxed::Box<dyn serialport::SerialPort>
}

impl DisplayController {
    fn set_brightness(&mut self, mut brightness: f32) {
        brightness = brightness - 100.0;
        let scaled_brightness = (brightness - 0.0) * ((950.0 - 10.0)/(100.0 - 0.0)) + 10.0;
        let command = format!("set brightness={scaled_brightness}", scaled_brightness=scaled_brightness);
        self.serial_port.write(command.as_bytes());
    }
}

fn controller_generator(port_path: &str) -> DisplayController {
    let serial_settings = SerialPortSettings {
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1),
    };
    DisplayController{
        brightness: 100,
        backlight: true,
        serial_settings: serial_settings,
        serial_port: serialport::open_with_settings(&port_path, &serial_settings).unwrap()
    }
}

fn main() {  
    let args: Vec<String> = env::args().collect();
    let brightness = &args[1].parse::<f32>().unwrap();
    let min_value: f32 = 0.0;
    let max_value: f32 = 100.0;
    if brightness < &min_value && brightness > &max_value {
        println!("Brightness must be between 0 and 100");
        std::process::exit(1)
    }

    let mut display_controller = controller_generator("/dev/ttyAMA0");

    display_controller.set_brightness(*brightness);
    println!("Set brightness to {}", brightness)
}