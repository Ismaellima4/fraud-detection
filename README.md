# Fraud Detection API - Rinha de Backend 2026

Uma API de detecção de fraudes de alta performance e ultra-baixa latência construída para o desafio da **Rinha de Backend 2026**. Este projeto implementa uma busca vetorial em tempo real (k-NN) para avaliar a probabilidade de fraude em transações operando estritamente dentro de recursos restritos (1 CPU base, 350MB de RAM).

## 🚀 Performance e Acurácia

Durante os testes de estresse (stress testing), esta aplicação estabeleceu números recordes:
- **Acurácia:** 100% (Zero Falsos Positivos, Zero Falsos Negativos).
- **Latência (P99):** Impressionantes **~6.17ms** (Varredura linear pura sobre 100.000 registros).
- **Estabilidade:** Taxa de falha (failure rate) de 0% sob o teto da carga máxima da CPU.
- **Score Final:** **5209.95** (Baseado na tabela de pesos e limites matemáticos oficiais da Rinha).

## 🛠 Tecnologias Utilizadas

- **Linguagem:** [Rust](https://www.rust-lang.org/)
- **Web Framework:** [Ohkami](https://github.com/ohkami-rs/ohkami) (v0.24)
- **Runtime:** [Monoio](https://github.com/bytedance/monoio) (io_uring native async)
- **Serialização:** [Serde](https://serde.rs/)
- **Infraestrutura:** Docker e Nginx (Balanceador de Carga)

## 🏗 Arquitetura "Zero-DB" (In-Memory)

Para respeitar as restrições extremas de infraestrutura e ausência de container para banco de dados relacional independente, o projeto adota o modelo puramente _In-Memory_:
- **Dataset Quantizado e Customizado:** Um script construtor localiza o JSON massivo estático e converte todos os dados crus em arquivos binários pré-digeridos — realizando um escalonamento dos decimais (`f32` [-1.0 a 1.0]) diretamente para inteiros (`u16` de 16-bits). O modelo de dados empurra o disco inteiro (100.000 perfis) em míseros **2.9MB** de tamanho bruto.
- **Localidade de Dados & Cache L3 (SIMD):** As estruturas na RAM operam com leitura paralela forçada sem Bounds Target-Checking do compilador (Pointer Chasing removido). O varrimento despeja os bytes serializados de maneira consecutiva e sem alocação ou cópias forçando o processador.
- **Busca Vetorial Agressiva:** k-Nearest Neighbors rodado limpo com alocação O(1), resolvendo em inteiros pesados e precisos a complexa verificação de **Distância Euclidiana** da vizinhança sem instabilidade de latência.

## 📋 Regras de Vetorização Computacional

Toda transação enviada é transformada num **Vetor de 14 Dimensões** que obedece severamente:
1. **Normalização Total:** Qualquer valor bruto vindo da Request é imprensado (clamped) matematicamente entre proporções variadas de `[0.0, 1.0]` segundo as diretrizes de médias globais do arquivo local `normalization.json`.
2. **Flags de Ausência (Sentinel Values):** Se a requisição carece de histórico (como `last_transaction: null`), as posições cardinais respectivas de tempo e distância assumem o hard code numérico `-1.0`.
3. **Defensor e Threshold (K=5):** Transações que sofrem avaliação recebem punição bloqueadora e retornam "fraud" logo que a taxa de aproximação se igualar ou superar um score bruto de $\ge 0.6$ na vizinhança quantizada de 5 transações mais análogas disponíveis in-memory.
