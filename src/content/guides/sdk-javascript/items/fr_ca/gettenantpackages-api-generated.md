## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| skip | number | Non |  |

## Réponse

Renvoie : [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTenantPackages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8421';
const packagesWithSkip: GetTenantPackages200Response = await getTenantPackages(tenantId, 25);
const packagesWithoutSkip: GetTenantPackages200Response = await getTenantPackages(tenantId);
[inline-code-end]

---