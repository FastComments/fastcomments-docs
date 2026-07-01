Agreguje dokumenty poprzez grupowanie ich (jeśli podano `groupBy`) i stosowanie wielu operacji. Obsługiwane są różne operacje (np. sum, countDistinct, avg itp.).

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| aggregationRequest | AggregationRequest | Tak |  |
| parentTenantId | string | Nie |  |
| includeStats | boolean | Nie |  |

## Odpowiedź

Zwraca: [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład agregacji'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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