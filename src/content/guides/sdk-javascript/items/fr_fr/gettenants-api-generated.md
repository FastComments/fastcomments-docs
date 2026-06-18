## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| meta | string | Non |  |
| skip | number | Non |  |

## Réponse

Retourne: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenants200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTenants'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fcom-tenant-8b4f2a1c";
const meta: string = "include=domains,billing&status=active";
const skip: number = 20;
const response: GetTenants200Response = await getTenants(tenantId, meta, skip);
console.log(response);
[inline-code-end]

---