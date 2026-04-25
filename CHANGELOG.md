# Changelog - Otimizações de Detecção Fraude API

Todas as otimizações sistêmicas com foco em performance bruta implementadas para quebrar a métrica e maximizar pontos na *Rinha de Backend 2026*.

## [Unreleased] - Sessão V2
*(Score Inicial de Baseline: ~4.000 pts | 80ms p99)*
*(Score Final Consolidado: 5209.95 | 6.17ms p99)*

### Added
- **Zero-Allocation no Parsing JSON:** Alterado o `TransactionPayload` para ler referências brutas `&'a str` das Requests do cliente. Usando tempo de vida do tratador Ohkami, zeramos requisições custosas de strings encadeadas sem depender da memória heap.
- **Quantização u16 Linear Baseada em Inteiros:** O Script re-gerador de dataset foi reconstruído para modelar uma régua perfeitamente mapeada entre coordenadas reais flutuantes (`[-1.0, 1.0]`) e valores de bits (`[0, 65535]`). Distâncias matemáticas persistem com precisão de 1/65535, removendo qualquer possibilidade de falso positivo e gerando pontapé em processamento integral rápido. 
- **Array Casting Forçado (Bounds Check Bypass):** Dentro da Hot Path em O(N), forçamos o compilador a absorver as features em forma padronizada declarando `&[u16; 14]`. Isto garantiu com que checamentos redundantes de overflow ficassem desabilitados na rotina de CPU.
- **State App Cache Partilhado (Arc):** O `AppState` isolou toda a leitura in-memory das strings e tabelas pré-formatadas num ponteiro isolado, aliviando o trânsito do I_Uring na porta de I/O em picos extremos. 

### Changed
- Refatoração dos modelos da detecção `calculate_fraud_score` para rodar purificação com Euclidiana Inteira (`diff * diff` via `u64`). Combinou zero erros e perdas mínimas de penalização do Ranking.
- **Tamanho Binário de Referência:** Redução massiva da pressão sobre o alocador de memória e cache da CPU ao limitar rigorosamente cada registro armazenado para apenas 29-bytes: o `references.bin` despencou para o micro-tamanho de 2.9 MB.  

### Removed
- **Padding Falso de Cache de L3:** A inserção de valores falsos 0.0 visando arredondar tamanho para 16 bytes foi testada, mapeada como devoradora de largura de banda e descartada em favor das otimizações dos inteiros puros.
- Remoção do tempo gasto com extração forçada de Float, garantindo distanciamento de cálculos lentos em transistores fpu de CPU base.
