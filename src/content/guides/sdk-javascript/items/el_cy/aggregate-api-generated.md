Συγκεντρώνει έγγραφα ομαδοποιώντας τα (αν groupBy παρέχεται) και εφαρμόζοντας πολλαπλές λειτουργίες.
Υποστηρίζονται διάφορες λειτουργίες (π.χ. sum, countDistinct, avg, κ.λπ.).

## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| aggregationRequest | AggregationRequest | Ναι |  |
| parentTenantId | string | Όχι |  |
| includeStats | boolean | Όχι |  |

## Απόκριση

Επιστρέφει: [`Aggregate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/Aggregate200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'aggregate Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_78a9';
const parentTenantId: string = 'parent_tenant_01';
const includeStats: boolean = true;
const aggregationRequest: AggregationRequest = {
  operation: { type: 'COUNT' },
  groupBy: ['pageUrl'],
  predicate: { field: 'status', operator: 'EQUALS', value: 'approved' },
  sort: [{ field: 'count', direction: 'DESC' }],
  limit: 25
};
const result: Aggregate200Response = await aggregate(tenantId, aggregationRequest, parentTenantId, includeStats);
[inline-code-end]