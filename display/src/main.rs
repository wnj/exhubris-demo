//! A task that operates as s SPI client.
//!
//! This task exists to test the SPI link. 

#![no_std]
#![no_main]

use hubris_task_slots::SLOTS;
use drv_stm32g0_sys_api::{Stm32G0Sys as Sys, Port};

#[export_name = "main"]
fn main() -> ! {
    // Create a client for the Sys driver and make our LED pin an output.
    let sys = Sys::from(SLOTS.sys);
    // LED L12
    sys.set_pin_output(Port::A, 0);

    // Record the current time so we can start our delay loop properly.
    let mut next_send = userlib::sys_get_timer().now;

    const INTERVAL: u64 = 500; // milliseconds

    loop {
        userlib::sys_set_timer(Some(next_send), hubris_notifications::TIMER_DISPLAY);

        // The proper thing to do, when waiting for a timer, is to sleep waiting
        // for notifications _and then check the time._ Otherwise other tasks
        // can wake you up by posting.
        loop {
            userlib::sys_recv_notification(hubris_notifications::TIMER_DISPLAY);
            let now = userlib::sys_get_timer().now;
            if now >= next_send {
                next_send += INTERVAL;
                break;
            }
        }

        sys.toggle_pin(Port::A, 0);
    } 
}
