## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| skip | number | No |  |

## Risposta

Restituisce: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getTenantPackages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-9f3b";
const packagesPage1: GetTenantPackagesResponse = await getTenantPackages(tenantId);
const packagesPage2: GetTenantPackagesResponse = await getTenantPackages(tenantId, 10);
[inline-code-end]

---