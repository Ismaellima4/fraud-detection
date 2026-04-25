mod detector;
mod handlers;
mod models;
mod vectorizer;

use crate::handlers::handle_fraud_score;
use crate::models::NormalizationConfig;
use axum::{
    routing::{get, post},
    Router,
};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub vectors: Arc<Vec<u16>>,
    pub labels: Arc<Vec<u8>>,
    pub mcc_risk: Arc<HashMap<String, f32>>,
    pub config: NormalizationConfig,
}

async fn handle_ready() -> &'static str {
    ""
}

#[tokio::main]
async fn main() {
    let config = NormalizationConfig::load();

    let mcc_risk_raw =
        fs::read_to_string("resources/mcc_risk.json").expect("Mcc risk file not found");
    let mcc_risk: HashMap<String, f32> = serde_json::from_str(&mcc_risk_raw).unwrap();

    let bin_data = fs::read("resources/references.bin").expect("Binary references not found");

    let num_records = bin_data.len() / 29;
    let mut vectors = Vec::with_capacity(num_records * 14);
    let mut labels = Vec::with_capacity(num_records);

    for chunk in bin_data.chunks_exact(29) {
        for j in 0..14 {
            let start = j * 2;
            vectors.push(u16::from_le_bytes([chunk[start], chunk[start + 1]]));
        }
        labels.push(chunk[28]);
    }

    let state = AppState {
        vectors: Arc::new(vectors),
        labels: Arc::new(labels),
        mcc_risk: Arc::new(mcc_risk),
        config,
    };

    let app = Router::new()
        .route("/ready", get(handle_ready))
        .route("/fraud-score", post(handle_fraud_score))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("Listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
