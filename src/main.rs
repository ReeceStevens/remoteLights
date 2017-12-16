extern crate wiringpi;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;
#[macro_use]
extern crate log;

use wiringpi::pin::Value::{High, Low};

use futures::{Future, Stream};
use hyper::{Client, Uri, Chunk};
use tokio_core::reactor::Core;
use serde_json::Value;

use std::thread;
use std::env;
use std::time::Duration;

type OutputPin = wiringpi::pin::OutputPin<wiringpi::pin::Gpio>;

const INTERVAL: u64 = 500;
let URL: &'static str = format!("http://{}/_status", &env::var("SERVER"));

pub fn toggle_pin(pin: &OutputPin) {
    pin.digital_write(High);
    thread::sleep(Duration::from_millis(INTERVAL));
    pin.digital_write(Low);
}

struct Button<> {
    on_pin: OutputPin,
    off_pin: OutputPin
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
            let v: Vec<bool> = serde_json::from_slice(&body).unwrap();
            Ok((v))
        })
    });
    core.run(request).unwrap()
}


struct Operation {
    idx: usize,
    action: bool
}

/// get_operations(local_status, remote_status)
///
/// Calculate the operations required to transform {local_status} to {remote_status}.
/// Returned as a vector of operations.
fn get_operations(local_status: &Vec<bool>, remote_status: &Vec<bool>) -> Vec<Operation> {
    let mut operations: Vec<Operation> = vec![];
    for (idx, local_stat) in local_status.iter().enumerate(){
        if *local_stat != remote_status[idx] {
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

    let mut local_status = vec![false, false, false];
    let buttons = vec![b1, b2, b3];

    loop {
        info!("Sleeping");
        thread::sleep(Duration::from_millis(INTERVAL));
        info!("Fetching remote status");
        let remote_status = get_status();
        info!("Remote status fetched: ");
        info!("{:?}", remote_status);
        let operations = get_operations(&local_status, &remote_status);
        info!("Calculated {} operations to be completed.", operations.len());
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
