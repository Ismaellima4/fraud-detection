pub fn calculate_fraud_score(input: &[f32; 14], references: &[([f32; 14], u8)]) -> f32 {
    let mut distances: Vec<(f32, u8)> = references
        .iter()
        .map(|(v, is_fraud)| {
            let mut dist = 0.0;
            for i in 0..14 {
                dist += (input[i] - v[i]).powi(2);
            }
            (dist, *is_fraud)
        })
        .collect();

    let k = 5;
    distances.select_nth_unstable_by(k - 1, |a, b| a.0.partial_cmp(&b.0).unwrap());

    let fraud_count = distances
        .iter()
        .take(k)
        .filter(|(_, is_fraud)| *is_fraud == 1)
        .count();

    fraud_count as f32 / k as f32
}
