Agrège les documents en les regroupant (si **groupBy** est fourni) et en appliquant plusieurs opérations.  
Différents types d’opérations (par ex. somme, **countDistinct**, moyenne, etc.) sont pris en charge.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| aggregationRequest | AggregationRequest | Oui |  |
| parentTenantId | string | Non |  |
| includeStats | boolean | Non |  |

## Réponse

Renvoie : [`AggregateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'agrégation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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