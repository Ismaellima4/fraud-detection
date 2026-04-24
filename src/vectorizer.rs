use crate::models::{NormalizationConfig, TransactionPayload};
use std::collections::HashMap;
use time::OffsetDateTime;

pub fn vectorize(
    payload: &TransactionPayload,
    config: &NormalizationConfig,
    mcc_risk: &HashMap<String, f32>,
) -> [f32; 14] {
    let mut v = [0.0; 14];

    let req_at = OffsetDateTime::parse(
        &payload.transaction.requested_at,
        &time::format_description::well_known::Rfc3339,
    )
    .unwrap_or(OffsetDateTime::UNIX_EPOCH);

    // 0: amount
    v[0] = (payload.transaction.amount / config.max_amount).clamp(0.0, 1.0);

    // 1: installments
    v[1] = (payload.transaction.installments as f32 / config.max_installments).clamp(0.0, 1.0);

    // 2: amount_vs_avg
    v[2] = ((payload.transaction.amount / payload.customer.avg_amount)
        / config.amount_vs_avg_ratio)
        .clamp(0.0, 1.0);

    // 3: hour_of_day (0-23)
    v[3] = req_at.hour() as f32 / 23.0;

    // 4: day_of_week (Mon=0, Sun=6)
    v[4] = (req_at.weekday().number_from_monday() - 1) as f32 / 6.0;

    // 5 & 6: minutes and km since last tx
    if let Some(last) = &payload.last_transaction {
        if let Ok(last_at) = OffsetDateTime::parse(
            &last.timestamp,
            &time::format_description::well_known::Rfc3339,
        ) {
            let diff = (req_at - last_at).whole_minutes() as f32;
            v[5] = (diff / config.max_minutes).clamp(0.0, 1.0);
        } else {
            v[5] = -1.0;
        }
        v[6] = (last.km_from_current / config.max_km).clamp(0.0, 1.0);
    } else {
        v[5] = -1.0;
        v[6] = -1.0;
    }

    // 7: km_from_home
    v[7] = (payload.terminal.km_from_home / config.max_km).clamp(0.0, 1.0);

    // 8: tx_count_24h
    v[8] = (payload.customer.tx_count_24h as f32 / config.max_tx_count_24h).clamp(0.0, 1.0);

    // 9: is_online
    v[9] = if payload.terminal.is_online { 1.0 } else { 0.0 };

    // 10: card_present
    v[10] = if payload.terminal.card_present {
        1.0
    } else {
        0.0
    };

    // 11: unknown_merchant
    let is_known = payload
        .customer
        .known_merchants
        .iter()
        .any(|m| m == &payload.merchant.id);
    v[11] = if is_known { 0.0 } else { 1.0 };

    // 12: mcc_risk
    v[12] = *mcc_risk.get(&payload.merchant.mcc).unwrap_or(&0.5);

    // 13: merchant_avg_amount
    v[13] = (payload.merchant.avg_amount / config.max_merchant_avg_amount).clamp(0.0, 1.0);

    v
}
