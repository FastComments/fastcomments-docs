## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | number | Nein |  |

## Antwort

Gibt zurück: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getTenantPackages Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f8e3b4c";
const skip: number = 20;
const packagesDefault: GetTenantPackages200Response = await getTenantPackages(tenantId);
const packagesWithSkip: GetTenantPackages200Response = await getTenantPackages(tenantId, skip);
[inline-code-end]

---