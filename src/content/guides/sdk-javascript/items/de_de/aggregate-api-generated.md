Aggregiert Dokumente, indem sie gruppiert werden (falls groupBy angegeben ist) und mehrere Operationen angewendet werden.
Verschiedene Operationen (z. B. sum, countDistinct, avg usw.) werden unterstützt.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| aggregationRequest | AggregationRequest | Ja |  |
| parentTenantId | string | Nein |  |
| includeStats | boolean | Nein |  |

## Antwort

Gibt zurück: [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'Aggregate-Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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