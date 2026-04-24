# Fraud Detection API - Rinha de Backend 2026

A high-performance, low-latency fraud detection API built for the **Rinha de Backend 2026** challenge. This project implements a real-time vector search (k-NN) to evaluate transaction fraud probability within strictly constrained resources (1 CPU, 350MB RAM).

## 🚀 Performance & Accuracy

During stress testing, this implementation achieved:
- **Accuracy:** 100% (Zero False Positives, Zero False Negatives).
- **Latency (p99):** ~92ms (Linear scan over 100,000 records).
- **Stability:** 0% failure rate under peak load.
- **Score:** **4033.84** (based on official Rinha scoring).

## 🛠 Tech Stack

- **Linguagem:** [Rust](https://www.rust-lang.org/)
- **Web Framework:** [Ohkami](https://github.com/ohkami-rs/ohkami) (v0.24)
- **Runtime:** [Monoio](https://github.com/bytedance/monoio) (io_uring based)
- **Serialization:** [Serde](https://serde.rs/)
- **Infrastructure:** Docker & Nginx (Load Balancer)

## 🏗 Architecture: "Zero-DB" (In-Memory)

To respect the extreme memory and CPU constraints, this API utilizes a "Zero-DB" approach:
- **Pre-processed Dataset:** A custom script converts the JSON dataset into a compact binary format (`references.bin`), reducing the memory footprint to only ~5.6MB for 100,000 records.
- **Data Locality:** Reference vectors are stored in contiguous memory to maximize CPU cache efficiency.
- **Efficient Search:** Implements a k-Nearest Neighbors (k-NN) search with $k=5$ using Euclidean distance.

## 📋 Vectorization Rules

Each transaction is transformed into a **14-dimensional vector** following strict normalization rules:
1. **Normalization:** Values are clamped between `[0.0, 1.0]` based on `normalization.json`.
2. **Sentinel Values:** Missing history (`last_transaction: null`) is represented by `-1` in the distance/time dimensions.
3. **Threshold:** Transactions are rejected if the fraud score (fraction of fraud labels in the 5 nearest neighbors) is $\ge 0.6$.
