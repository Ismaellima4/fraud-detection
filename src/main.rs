mod detector;
mod handlers;
mod models;
mod vectorizer;

use crate::handlers::handle_fraud_score;
use crate::models::NormalizationConfig;
use ohkami::claw::status;
use ohkami::fang::Context;
use ohkami::prelude::*;
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

impl ohkami::FangAction for AppState {}

async fn handle_ready() -> status::NoContent {
    status::NoContent
}

#[monoio::main(timer = true)]
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

    let ohkami = Ohkami::new((
        Context::new(state),
        "/ready".GET(handle_ready),
        "/fraud-score".POST(handle_fraud_score),
    ));

    ohkami.howl("0.0.0.0:3000").await
}
