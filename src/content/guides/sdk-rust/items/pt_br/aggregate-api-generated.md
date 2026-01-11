Agrega documentos agrupando-os (se groupBy for fornecido) e aplicando múltiplas operações.
Diferentes operações (por exemplo sum, countDistinct, avg, etc.) são suportadas.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenant_id | String | Sim |  |
| aggregation_request | models::AggregationRequest | Sim |  |
| parent_tenant_id | String | Não |  |
| include_stats | bool | Não |  |

## Resposta

Retorna: [`AggregationResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregation_response.rs)

---