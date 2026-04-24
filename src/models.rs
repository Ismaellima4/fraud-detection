#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TransactionPayload {
    pub id: String,
    pub transaction: TransactionInfo,
    pub customer: CustomerInfo,
    pub merchant: MerchantInfo,
    pub terminal: TerminalInfo,
    pub last_transaction: Option<LastTransactionInfo>,
}

#[derive(Deserialize)]
pub struct TransactionInfo {
    pub amount: f32,
    pub installments: u8,
    pub requested_at: String,
}

#[derive(Deserialize)]
pub struct CustomerInfo {
    pub avg_amount: f32,
    pub tx_count_24h: u32,
    pub known_merchants: Vec<String>,
}

#[derive(Deserialize)]
pub struct MerchantInfo {
    pub id: String,
    pub mcc: String,
    pub avg_amount: f32,
}

#[derive(Deserialize)]
pub struct TerminalInfo {
    pub is_online: bool,
    pub card_present: bool,
    pub km_from_home: f32,
}

#[derive(Deserialize)]
pub struct LastTransactionInfo {
    pub timestamp: String,
    pub km_from_current: f32,
}

#[derive(Serialize)]
pub struct ResponsePayload {
    pub approved: bool,
    pub fraud_score: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NormalizationConfig {
    pub max_amount: f32,
    pub max_installments: f32,
    pub max_tx_count_24h: f32,
    pub max_km: f32,
    pub max_minutes: f32,
    pub amount_vs_avg_ratio: f32,
    pub max_merchant_avg_amount: f32,
}

impl NormalizationConfig {
    pub fn load() -> Self {
        let config_raw = std::fs::read_to_string("resources/normalization.json")
            .expect("Normalization file not found");
        serde_json::from_str(&config_raw).expect("Failed to parse normalization.json")
    }
}
