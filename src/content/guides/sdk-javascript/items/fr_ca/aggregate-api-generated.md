---
Agrège des documents en les groupant (si groupBy est fourni) et en appliquant plusieurs opérations. Différentes opérations (p. ex. sum, countDistinct, avg, etc.) sont prises en charge.

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| aggregationRequest | AggregationRequest | Oui |  |
| parentTenantId | string | Non |  |
| includeStats | boolean | Non |  |

## Réponse

Retourne: [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'agrégation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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