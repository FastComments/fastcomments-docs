---
Aggrega documenti raggruppandoli (se viene fornito groupBy) e applicando più operazioni.
Sono supportate diverse operazioni (ad es. sum, countDistinct, avg, ecc.).

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| aggregationRequest | AggregationRequest | Sì |  |
| parentTenantId | string | No |  |
| includeStats | boolean | No |  |

## Risposta

Restituisce: [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di aggregate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_72b3";
const parentTenantId: string = "parent_acme_corp";
const aggregationRequest: AggregationRequest = {
  groupBy: ["postId"],
  predicates: [
    { field: "status", operator: "EQ", value: { stringValue: "published" } as QueryPredicateValue }
  ],
  operations: [
    { type: AggregationOpType.COUNT, field: "id", alias: "commentCount" } as AggregationOperation
  ],
  sort: [{ field: "commentCount", direction: "DESC" } as AggregationRequestSort],
  limit: 25
};
const response: AggregateResponse = await aggregate(tenantId, aggregationRequest, parentTenantId, true);
[inline-code-end]

---