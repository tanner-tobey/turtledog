// docs/examples/concurrency.bb
//
// This example demonstrates BlueBird's safe and simple concurrency model.
// A 'Channel' is used to send data between two lightweight tasks running
// in parallel, preventing data races by design.

fn main() {
    // Create a new channel. `tx` is the sender, `rx` is the receiver.
    let (tx, rx) = Channel<string>::new()

    print("Spawning a new task...")
    spawn {
        // This block of code runs in a new concurrent task.
        // The `tx` variable is 'moved' into this task by ownership rules.
        tx.send("Hello from the spawned task!")
    }

    // The main task will block here until a message is received on `rx`.
    let received_message = rx.recv()

    print(f"The main task received a message: '{received_message}'")
    print("Program finished.")
}
