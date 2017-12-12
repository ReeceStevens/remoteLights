extern crate wiringpi;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;

use wiringpi::pin::Value::{High, Low};
use wiringpi::pin::Pin;

use futures::{Future, Stream};
use hyper::{Client, Uri, Chunk};
use tokio_core::reactor::Core;
use serde_json::Value;

use std::{thread, time};

const INTERVAL: u8 = 500;
const URL: &'static str = "http://174.138.64.189/_status";
// const URL: &'static str = "http://127.0.0.1:5000/_status";

pub<T: Pin> fn toggle_pin(pin: &T) {
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

fn get_status() -> Vec<bool> {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let url: Uri = URL.parse().unwrap();
    let request = client.get(url).and_then(|res| {
        res.body().concat2().and_then(move |body: Chunk| {
            let v: Value = serde_json::from_slice(&body).unwrap();
            Ok((v))
        })
    });
    core.run(request).unwrap()
}


struct Operation {
    idx: u8,
    action: bool
}

/// get_operations(local_status, remote_status)
///
/// Calculate the operations required to transform {local_status} to {remote_status}.
/// Returned as a vector of operations.
fn get_operations(local_status: &Vec<bool>, remote_status: &Vec<bool>) -> Vec<Operation> {
    let mut operations: Vec<Operation> = vec![];
    for (idx, local_stat) in local_status.iter().enumerate)({
        if local_stat != remote_status[idx] {
            operations.push(Operation {
                idx: idx,
                action: !local_stat
            });
        }
    }
    operations
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

    local_status = vec![false, false, false];
    buttons = vec![b1, b2, b3];

    loop {
        thread::sleep(INTERVAL);
        let remote_status: Vec<bool> = get_status();
        let mut operations = get_operations(&local_status, &remote_status);

        for operation in operations.iter() {
            if operation.action {
                buttons[operation.idx].on();
            } else {
                buttons[operation.idx].off();
            }
            local_status[operation.idx] = operation.action;
        }

    }
}
