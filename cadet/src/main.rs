use std::{collections::HashMap, sync::{Arc, Mutex}};

use flume::unbounded;
use metrics_exporter_prometheus::PrometheusBuilder;

use rocketman::{
    connection::JetstreamConnection, handler, ingestion::LexiconIngestor, options::JetstreamOptions,
};

mod db;
mod ingestors;

fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

fn setup_metrics() {
    // Initialize metrics here
    if let Err(e) = PrometheusBuilder::new().install() {
        eprintln!(
            "Failed to install, program will run without Prometheus exporter: {}",
            e
        );
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    setup_tracing();
    setup_metrics();

    let pool = db::init_pool()
        .await
        .expect("Could not get PostgreSQL pool");

    let (msg_tx, msg_rx) = unbounded();

    let opts = JetstreamOptions::builder()
        .wanted_collections(vec!["fm.teal.alpha.feed.play".to_string()])
        .build();

    let jetstream = JetstreamConnection::new(opts);

    let mut ingestors: HashMap<String, Box<dyn LexiconIngestor + Send + Sync>> = HashMap::new();

    ingestors.insert(
        "fm.teal.alpha.feed.play".to_string(),
        Box::new(ingestors::teal::feed_play::PlayIngestor::new(pool)),
    );

    // tracks the last message we've processed
    // TODO: read from db/config so we can resume from where we left off in case of crash
    let cursor: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    // Spawn a task to process messages from the queue.
    // bleh at this clone
    let c_cursor = cursor.clone();
    tokio::spawn(async move {
        while let Ok(message) = msg_rx.recv_async().await {
            if let Err(e) = handler::handle_message(message, &ingestors, c_cursor.clone()).await {
                eprintln!("Error processing message: {}", e);
            };
        }
    });

    if let Err(e) = jetstream.connect(msg_tx, cursor).await {
        eprintln!("Failed to connect to Jetstream: {}", e);
        std::process::exit(1);
    }
}
