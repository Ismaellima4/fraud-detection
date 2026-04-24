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
    pub references: Arc<Vec<([f32; 14], u8)>>,
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

    let mut references = Vec::with_capacity(100_000);
    for chunk in bin_data.chunks_exact(57) {
        let mut v = [0.0f32; 14];
        for (val, bytes_chunk) in v.iter_mut().zip(chunk[..56].chunks_exact(4)) {
            *val = f32::from_le_bytes(bytes_chunk.try_into().unwrap());
        }
        references.push((v, chunk[56]));
    }

    let state = AppState {
        references: Arc::new(references),
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
