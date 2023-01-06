#![no_std]

#[cfg(test)]
mod tests;

mod io;
use io::{FlipperAction, FlipperEvent};

use gstd::prelude::*;

gstd::metadata! {
  title: "flipper",
  handle:
    input: FlipperAction,
    output: FlipperEvent,
}

static mut FlipperState: bool = false;

#[no_mangle]
unsafe extern "C" fn handle() {
    let action: FlipperAction = gstd::msg::load().expect("failed to load input message");
    match action {
        FlipperAction::Flip => {
            FlipperState = !FlipperState;
            let event = FlipperEvent::FlippedTo(FlipperState as u8);
            gstd::msg::reply(event, 0).expect("failed to send response");
        },
    }
}
