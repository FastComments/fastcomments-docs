## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| meta | string | Non |  |
| skip | number | Non |  |

## Réponse

Renvoie: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'getTenants Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-72b';
const meta: string = 'include=domains,billing';
const skip: number = 20;
const result: GetTenantsResponse = await getTenants(tenantId, meta, skip);
[inline-code-end]

---