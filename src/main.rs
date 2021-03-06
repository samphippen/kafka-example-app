extern crate kafka;

use kafka::consumer::{Consumer, FetchOffset};
use kafka::error::Error as KafkaError;

/// This program demonstrates consuming messages through a `Consumer`.
/// This is a convenient client that will fit most use cases.  Note
/// that messages must be marked and commited as consumed to ensure
/// only once delivery.
fn main() {
    let broker = "192.168.99.101".to_owned();
    let topic = "test".to_owned();
    let group = "consumers".to_owned();

    if let Err(e) = consume_messages(group, topic, vec![broker]) {
        println!("Failed consuming messages: {}", e);
    }
}

fn consume_messages(group: String, topic: String, brokers: Vec<String>)
                    -> Result<(), KafkaError>
{
    let mut con = try!(Consumer::from_hosts(brokers, group, topic)
                       .with_fallback_offset(FetchOffset::Earliest)
                       .create());

    loop {
        let mss = try!(con.poll());
        if mss.is_empty() {
            println!("No messages available right now.");
            return Ok(());
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                println!("{}:{}@{}: {:?}", ms.topic(), ms.partition(), m.offset, m.value);
            }
            con.consume_messageset(ms);
        }
        try!(con.commit_consumed());
    }
}
