Agrega documentos agrupando‑os (se **groupBy** for fornecido) e aplicando múltiplas operações.  
Diferentes operações (por exemplo, sum, countDistinct, avg, etc.) são suportadas.

## Parameters

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| aggregationRequest | AggregationRequest | Sim |  |
| parentTenantId | string | Não |  |
| includeStats | boolean | Não |  |

## Response

Retorna: [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

## Example

[inline-code-attrs-start title = 'Exemplo de agregação'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-12345";

const aggregationRequest: AggregationRequest = {
  predicates: [
    {
      field: "status",
      operator: "eq",
      value: { type: "string", value: "approved" }
    }
  ],
  operations: [
    { type: "count", field: "commentId" }
  ],
  sort: { field: "createdAt", direction: "desc" }
};

const parentTenantId: string = "parent-001";
const includeStats: boolean = true;

const result: AggregateResponse = await aggregate(
  tenantId,
  aggregationRequest,
  parentTenantId,
  includeStats
);
[inline-code-end]