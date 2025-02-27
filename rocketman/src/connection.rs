use flume::{Receiver, Sender};
use futures_util::StreamExt;
use metrics::{counter, describe_counter, describe_histogram, histogram, Unit};
use std::cmp::{max, min};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{error, info};
use url::Url;

use crate::options::JetstreamOptions;

pub struct JetstreamConnection {
    pub opts: JetstreamOptions,
    reconnect_tx: flume::Sender<()>,
    reconnect_rx: flume::Receiver<()>,
    msg_tx: flume::Sender<Message>,
    msg_rx: flume::Receiver<Message>,
}

impl JetstreamConnection {
    pub fn new(opts: JetstreamOptions) -> Self {
        let (reconnect_tx, reconnect_rx) = flume::unbounded();
        let (msg_tx, msg_rx) = flume::unbounded();
        Self {
            opts,
            reconnect_tx,
            reconnect_rx,
            msg_tx,
            msg_rx,
        }
    }

    pub fn get_reconnect_tx(&self) -> Sender<()> {
        self.reconnect_tx.clone()
    }

    pub fn get_msg_rx(&self) -> Receiver<Message> {
        self.msg_rx.clone()
    }

    fn build_ws_url(&self, cursor: Arc<Mutex<Option<String>>>) -> String {
        let mut url = Url::parse(&self.opts.ws_url.to_string()).unwrap();

        // Append query params
        if let Some(ref cols) = self.opts.wanted_collections {
            for col in cols {
                url.query_pairs_mut().append_pair("wantedCollections", col);
            }
        }
        if let Some(ref dids) = self.opts.wanted_dids {
            for did in dids {
                url.query_pairs_mut().append_pair("wantedDids", did);
            }
        }
        if let Some(cursor) = cursor.lock().unwrap().as_ref() {
            url.query_pairs_mut().append_pair("cursor", cursor);
        }

        url.to_string()
    }

    pub async fn connect(
        &self,
        cursor: Arc<Mutex<Option<String>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        describe_counter!(
            "jetstream.connection.attempt",
            Unit::Count,
            "attempts to connect to jetstream service"
        );
        describe_counter!(
            "jetstream.connection.error",
            Unit::Count,
            "errors connecting to jetstream service"
        );
        describe_histogram!(
            "jetstream.connection.duration",
            Unit::Seconds,
            "Time connected to jetstream service"
        );
        describe_counter!(
            "jetstream.connection.reconnect",
            Unit::Count,
            "reconnects to jetstream service"
        );
        let mut retry_interval = 1;

        loop {
            counter!("jetstream.connection.attempt").increment(1);
            info!("Connecting to {}", self.opts.ws_url);
            let start = Instant::now();

            let ws_url = self.build_ws_url(cursor.clone());

            match connect_async(ws_url).await {
                Ok((ws_stream, response)) => {
                    let elapsed = start.elapsed();
                    info!("Connected. HTTP status: {}", response.status());

                    let (_, mut read) = ws_stream.split();
                    retry_interval = 1;

                    loop {
                        // Inner loop to handle messages and reconnect signals
                        tokio::select! {
                            message_result = read.next() => {
                                match message_result {
                                    Some(message) => {
                                        histogram!("jetstream.connection.duration").record(elapsed.as_secs_f64());
                                        match message {
                                            Ok(message) => {
                                                if let Err(err) = self.msg_tx.send_async(message).await {
                                                    counter!("jetstream.error").increment(1);
                                                    error!("Failed to queue message: {}", err);
                                                }
                                            }
                                            Err(e) => {
                                                    counter!("jetstream.error").increment(1);
                                                    error!("Error: {}", e);
                                            }
                                        }
                                    }
                                    None => {
                                        info!("Stream closed by server.");
                                        // technically this should be a 'break' but it results in a reconnect
                                        counter!("jetstream.connection.reconnect").increment(1);
                                        break; // Stream ended, break inner loop to reconnect
                                    }
                                }
                            }
                            _ = self.reconnect_rx.recv_async() => {
                                info!("Reconnect signal received.");
                                counter!("jetstream.connection.reconnect").increment(1);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    counter!("jetstream.connection.error").increment(1);
                    eprintln!("Connection error: {}", e);
                }
            }

            let sleep_time = max(1, min(self.opts.max_retry_interval, retry_interval));
            info!("Reconnecting in {} seconds...", sleep_time);
            sleep(Duration::from_secs(sleep_time)).await;
            retry_interval = retry_interval * 2;
        }
    }

    pub fn force_reconnect(&self) -> Result<(), flume::SendError<()>> {
        info!("Force reconnect requested.");
        self.reconnect_tx.send(()) // Send a reconnect signal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio::task;
    use tokio::time::{timeout, Duration};
    use tokio_tungstenite::tungstenite::Message;

    #[test]
    fn test_build_ws_url() {
        let opts = JetstreamOptions {
            wanted_collections: Some(vec!["col1".to_string(), "col2".to_string()]),
            wanted_dids: Some(vec!["did1".to_string()]),
            ..Default::default()
        };
        let connection = JetstreamConnection::new(opts);

        let test = Arc::new(Mutex::new(Some("test".to_string())));

        let url = connection.build_ws_url(test);

        assert!(url.starts_with("wss://"));
        assert!(url.contains("cursor=test"));
        assert!(url.contains("wantedCollections=col1"));
        assert!(url.contains("wantedCollections=col2"));
        assert!(url.contains("wantedDids=did1"));
    }

    #[tokio::test]
    async fn test_force_reconnect() {
        let opts = JetstreamOptions::default();
        let connection = JetstreamConnection::new(opts);

        // Spawn a task to listen for the reconnect signal
        let reconnect_rx = connection.reconnect_rx.clone();
        let recv_task = task::spawn(async move {
            reconnect_rx
                .recv_async()
                .await
                .expect("Failed to receive reconnect signal");
        });

        connection
            .force_reconnect()
            .expect("Failed to send reconnect signal");

        // Ensure reconnect signal was received
        assert!(recv_task.await.is_ok());
    }

    #[tokio::test]
    async fn test_message_queue() {
        let opts = JetstreamOptions::default();
        let connection = JetstreamConnection::new(opts);

        let msg_rx = connection.get_msg_rx();
        let msg = Message::Text("test message".into());

        // Send a message to the queue
        connection
            .msg_tx
            .send_async(msg.clone())
            .await
            .expect("Failed to send message");

        // Receive and verify the message
        let received = msg_rx
            .recv_async()
            .await
            .expect("Failed to receive message");
        assert_eq!(received, msg);
    }

    #[tokio::test]
    async fn test_connection_retries_on_failure() {
        let opts = JetstreamOptions::default();
        let connection = Arc::new(JetstreamConnection::new(opts));

        let cursor = Arc::new(Mutex::new(None));

        // Timeout to prevent infinite loop
        let result = timeout(Duration::from_secs(3), connection.connect(cursor)).await;

        assert!(result.is_err(), "Expected timeout due to retry logic");
    }
}
