Aggrega i documenti raggruppandoli (se viene fornito groupBy) e applicando più operazioni.
Sono supportate diverse operazioni (ad es. sum, countDistinct, avg, ecc.).

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| aggregationRequest | AggregationRequest | Sì |  |
| parentTenantId | string | No |  |
| includeStats | boolean | No |  |

## Risposta

Restituisce: [`Aggregate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/Aggregate200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di aggregate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---