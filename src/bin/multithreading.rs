use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
	let (tx, mut rx) = mpsc::channel(1024); // After 1024 messages, the channel will be blocked.
	// rx.send.await will be sleep until the channel is not full.

	// The first 5 message are stored into the channel, and after 5 seconds of load, will get message from loaded buffer.
	tokio::spawn(async move {
		sleep(Duration::from_millis(5000)).await;
		loop {
			let msg = rx.recv().await;
			println!("Got: {}", msg.unwrap());
		}
	});

	tokio::spawn(async move {
		loop {
			tx.send("Hello").await.unwrap();
			sleep(Duration::from_millis(1000)).await;
		}
	});

	loop {
		// Looping to not close the session.
		sleep(Duration::from_millis(1000)).await;
	}
}