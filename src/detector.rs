pub fn calculate_fraud_score(input: &[f32; 14], vectors: &[f32], labels: &[u8]) -> f32 {
    const K: usize = 5;
    let mut best_dist_sq = [f32::MAX; K];
    let mut best_labels = [0u8; K];

    let n = labels.len();
    for i in 0..n {
        let v_offset = i * 14;
        let v = &vectors[v_offset..v_offset + 14];
        
        let mut dist_sq = 0.0;
        for j in 0..14 {
            let diff = input[j] - v[j];
            dist_sq += diff * diff;
        }

        if dist_sq < best_dist_sq[K - 1] {
            let mut pos = K - 1;
            while pos > 0 && best_dist_sq[pos - 1] > dist_sq {
                best_dist_sq[pos] = best_dist_sq[pos - 1];
                best_labels[pos] = best_labels[pos - 1];
                pos -= 1;
            }
            best_dist_sq[pos] = dist_sq;
            best_labels[pos] = labels[i];
        }
    }

    let fraud_count = best_labels.iter().filter(|&&l| l == 1).count();
    fraud_count as f32 / K as f32
}
