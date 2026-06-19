## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | number | Nein |  |

## Antwort

Gibt zurück: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getTenantPackages Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-9f3b";
const packagesPage1: GetTenantPackagesResponse = await getTenantPackages(tenantId);
const packagesPage2: GetTenantPackagesResponse = await getTenantPackages(tenantId, 10);
[inline-code-end]

---