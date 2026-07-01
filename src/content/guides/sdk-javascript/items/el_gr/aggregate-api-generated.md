Συγκεντρώνει έγγραφα ομαδοποιώντας τα (εάν παρέχεται το groupBy) και εφαρμόζοντας πολλαπλές λειτουργίες. Υποστηρίζονται διαφορετικές λειτουργίες (π.χ. sum, countDistinct, avg κ.λπ.).

## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Yes |  |
| aggregationRequest | AggregationRequest | Yes |  |
| parentTenantId | string | No |  |
| includeStats | boolean | No |  |

## Response

Returns: [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα Συγκέντρωσης'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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