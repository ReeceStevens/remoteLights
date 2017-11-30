extern crate wiringpi;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use wiringpi::pin::Value::{High, Low};
use wiringpi::pin::Pin;

use futures::{Future};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

use std::{thread, time};

const INTERVAL: u8 = 500;

pub fn<T: Pin> toggle_pin(pin: &T) {
    pin.digital_write(High);
    thread::sleep(INTERVAL);
    pin.digital_write(Low);
}

struct<T: Pin> Button {
    on_pin: T,
    off_pin: T
}

impl Button {
    pub fn on(&self) {
        toggle_pin(&self.on_pin)
    }
    pub fn off(&self) {
        toggle_pin(&self.off_pin)
    }
}

fn main() {
    //Setup WiringPi with its own pin numbering order
    let pi = wiringpi::setup_gpio();

    let b1 = Button {
        on_pin: pi.output_pin(22),
        off_pin: pi.output_pin(25)
    };
    let b2 = Button {
        on_pin: pi.output_pin(19),
        off_pin: pi.output_pin(12)
    };
    let b3 = Button {
        on_pin: pi.output_pin(6),
        off_pin: pi.output_pin(21)
    };

    loop {
        b1.on();
        thread::sleep(INTERVAL);
        b1.off();
        thread::sleep(INTERVAL);
        b2.on();
        thread::sleep(INTERVAL);
        b2.off();
        thread::sleep(INTERVAL);
        b3.on();
        thread::sleep(INTERVAL);
        b3.off();
        thread::sleep(INTERVAL);
    }
}
