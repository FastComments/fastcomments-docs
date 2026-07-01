Agregira dokumente grupisanjem (ako je naveden groupBy) i primenom više operacija. Podržane su različite operacije (npr. sum, countDistinct, avg, itd.).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | Yes |  |
| parentTenantId | string | No |  |
| includeStats | boolean | No |  |

## Odgovor

Vraća: [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

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