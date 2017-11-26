extern crate wiringpi;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use wiringpi::pin::Value::{High, Low};

use futures::{Future};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

use std::{thread, time};

fn main() {
    //Setup WiringPi with its own pin numbering order
    let pi = wiringpi::setup();

    //Use WiringPi pin 0 as output
    let pin = pi.output_pin(22);

    let interval = time::Duration::from_millis(1000);

    loop {
        //Set pin 0 to high and wait one second
        pin.digital_write(High);
        thread::sleep(interval);

        //Set pin 0 to low and wait one second
        pin.digital_write(Low);
        thread::sleep(interval);
    }
}
