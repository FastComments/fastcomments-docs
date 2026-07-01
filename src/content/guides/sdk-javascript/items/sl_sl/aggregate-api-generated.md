Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.  
Različne operacije (npr. sum, countDistinct, avg, itd.) so podprte.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | Yes |  |
| parentTenantId | string | No |  |
| includeStats | boolean | No |  |

## Odgovor

Vrne: [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer agregacije'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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