Agrega documentos agrupándolos (si se proporciona groupBy) y aplicando múltiples operaciones. Se admiten diferentes operaciones (por ejemplo, sum, countDistinct, avg, etc.).

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| aggregationRequest | AggregationRequest | Sí |  |
| parentTenantId | string | No |  |
| includeStats | boolean | No |  |

## Respuesta

Devuelve: [`Aggregate200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/Aggregate200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de aggregate'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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