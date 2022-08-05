/*
    Objective:
    Blinks an LED a few times.

    Usage:
    Intended to be run an init system such as system at boot time to indicate to the user that the boot is successful.

    Hardware:
    Assumes Raspberry Pi 4, Connected to GPIO 7

    Author:
    Satyam

    Reference:
    https://github.com/golemparts/rppal
*/

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

extern crate syslog;
#[macro_use]
extern crate log;

use syslog::{Facility, Formatter3164, BasicLogger};
use log::{SetLoggerError, LevelFilter};

// Gpio uses BCM pin numbering. BCM GPIO 7 is tied to physical pin 26.
const GPIO_LED: u8 = 7;

fn main() -> Result<(), Box<dyn Error>> {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "mfab-status-led".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
            .map(|()| log::set_max_level(LevelFilter::Info));

    info!("Microfabrication Status LED program started.");

    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    let mut count = 0u32;
    let blink_duration = 500u64;

    // Blink the LED by setting the pin's logic level high for the given ms (blink_duration).
    loop {
        count += 1;

        pin.set_low();
        thread::sleep(Duration::from_millis(blink_duration));
        pin.set_high();
        thread::sleep(Duration::from_millis(blink_duration));

        info!("Count: {}", count);

        if count == 5 {
            // Exit the loop
            break;
        }
    }

    info!("Program is about to terminate.");

    Ok(())
}