Agrega documentos agrupando-os (se groupBy for fornecido) e aplicando múltiplas operações.
Diferentes operações (por exemplo, sum, countDistinct, avg, etc.) são suportadas.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| aggregationRequest | AggregationRequest | Sim |  |
| parentTenantId | string | Não |  |
| includeStats | boolean | Não |  |

## Resposta

Retorna: [`AggregationResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregationResponse.ts)

---