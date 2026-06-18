## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| meta | string | Nein |  |
| skip | number | Nein |  |

## Antwort

Gibt zurück: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenants200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getTenants Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "fcom-tenant-8b4f2a1c";
const meta: string = "include=domains,billing&status=active";
const skip: number = 20;
const response: GetTenants200Response = await getTenants(tenantId, meta, skip);
console.log(response);
[inline-code-end]

---