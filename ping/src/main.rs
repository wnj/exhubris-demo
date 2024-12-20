//! A task that spams its counterpart, pong, with IPC messages.
//!
//! This task exists to test the build system and IPC implementation. You
//! probably don't want to include it in a real application.

#![no_std]
#![no_main]

use hubris_task_slots::SLOTS;
use drv_stm32g0_sys_api::{Stm32G0Sys as Sys, Port};
use pong_api::Pong;

#[export_name = "main"]
fn main() -> ! {
    // Create a client for the Sys driver and make our LED pin an output.
    let sys = Sys::from(SLOTS.sys);
    sys.set_pin_output(Port::C, 14); // RED led

    // Create a client for the Pong task.
    let pong = Pong::from(SLOTS.pong);

    // Record the current time so we can start our delay loop properly.
    let mut next_send = userlib::sys_get_timer().now;

    const INTERVAL: u64 = 5; // milliseconds
    let mut send_count = 0;

    loop {
        userlib::sys_set_timer(Some(next_send), hubris_notifications::TIMER_PING);

        // The proper thing to do, when waiting for a timer, is to sleep waiting
        // for notifications _and then check the time._ Otherwise other tasks
        // can wake you up by posting.
        loop {
            userlib::sys_recv_notification(hubris_notifications::TIMER_PING);
            let now = userlib::sys_get_timer().now;
            if now >= next_send {
                next_send += INTERVAL;
                break;
            }
        }

        // Send a message and update our counter.
        pong.pong();
        send_count += 1;

        // Arrange to blink the LED by toggling it every 100 sends.
        if send_count == 100 {
            send_count = 0;
            sys.toggle_pin(Port::C, 14);
        }
    }
}
