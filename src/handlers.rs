use crate::AppState;
use crate::detector::calculate_fraud_score;
use crate::models::{ResponsePayload, TransactionPayload};
use crate::vectorizer::vectorize;
use axum::{extract::State, Json};

pub async fn handle_fraud_score(
    State(state): State<AppState>,
    Json(payload): Json<TransactionPayload>,
) -> Json<ResponsePayload> {
    let vector = vectorize(&payload, &state.config, &state.mcc_risk);
    let fraud_score = calculate_fraud_score(&vector, &state.vectors, &state.labels);

    Json(ResponsePayload {
        approved: fraud_score < 0.6,
        fraud_score,
    })
}
