use crate::AppState;
use crate::detector::calculate_fraud_score;
use crate::models::{ResponsePayload, TransactionPayload};
use crate::vectorizer::vectorize;
use ohkami::claw::Json;
use ohkami::fang::Context;

pub async fn handle_fraud_score(
    Json(payload): Json<TransactionPayload>,
    Context(state): Context<'_, AppState>,
) -> Json<ResponsePayload> {
    let vector = vectorize(&payload, &state.config, &state.mcc_risk);
    let fraud_score = calculate_fraud_score(&vector, &state.references);

    Json(ResponsePayload {
        approved: fraud_score < 0.6,
        fraud_score,
    })
}
