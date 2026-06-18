Belgeleri (eğer groupBy sağlanmışsa) gruplandırarak ve birden çok işlem uygulayarak toplar.
Farklı işlemler (ör. sum, countDistinct, avg, vb.) desteklenir.

## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| aggregationRequest | AggregationRequest | Evet |  |
| parentTenantId | string | Hayır |  |
| includeStats | boolean | Hayır |  |

## Response

Döndürür: [`Aggregate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/Aggregate200Response.ts)

## Örnek

[inline-code-attrs-start title = 'aggregate Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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