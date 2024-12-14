//! A really basic IPC server that also occasionally crashes itself.

#![no_std]
#![no_main]

#[export_name = "main"]
fn main() -> ! {
    // Space for incoming messages; the `PONG_BUFFER_SIZE` constant is generated
    // (see below).
    let mut buffer = [core::mem::MaybeUninit::uninit(); PONG_BUFFER_SIZE];
    // Placeholder server state struct.
    let mut server = Server { message_count: 0 };

    loop {
        idyll_runtime::dispatch(&mut server, &mut buffer);

        // Periodically crash this task to test both supervisor restarts, and
        // IPC client handling of dead codes.
        if server.message_count % 10 == 0 {
            userlib::sys_panic(b"");
        }
    }
}

// In a fancier server, this would contain application state.
struct Server {
    message_count: usize,
}

impl Pong for Server {
    fn pong(
        &mut self,
        _: idyll_runtime::Meta,
    ) -> Result<(), userlib::ReplyFaultReason> {
        self.message_count = self.message_count.wrapping_add(1);
        Ok(())
    }
}

// This loads in the generated server support code produced by our build.rs and
// idyll.
include!(concat!(env!("OUT_DIR"), "/generated_server.rs"));
