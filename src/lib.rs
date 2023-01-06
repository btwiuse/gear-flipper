#![no_std]

#[cfg(test)]
mod tests;

mod io;
use io::{Action, Event};

use gstd::prelude::*;

gstd::metadata! {
  title: "flipper",
  handle:
    input: Action,
    output: Event,
}

static mut STATE: bool = false;

#[no_mangle]
extern "C" fn handle() {
    let action: Action = gstd::msg::load().expect("failed to load input message");
    match action {
        Action::Flip => unsafe {
            STATE = !STATE;
            let event = Event::FlippedTo(STATE as u8);
            gstd::msg::reply(event, 0).expect("failed to send response");
        },
    }
}
