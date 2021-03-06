use mini_redis::client;
use tokio::sync::mpsc;

use bytes::Bytes;

#[derive(Debug)]
enum Command {
	Get { key: String },
	Set { key: String, val: Bytes },
}

#[tokio::main]
async fn main() {
	let (tx, mut rx) = mpsc::channel(32);
	let tx2 = tx.clone();

	tokio::spawn(async move {
		tx.send("sending from first handle").await;
	});

	tokio::spawn(async move {
		tx2.send("sending from second handle").await;
	});

	while let Some(message) = rx.recv().await {
		println!("GOT = {}", message);
	}

}
