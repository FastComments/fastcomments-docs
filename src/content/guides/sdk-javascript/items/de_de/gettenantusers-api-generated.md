## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| skip | number | Nein |  |

## Antwort

Rückgabe: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsersResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getTenantUsers Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const skip: number = 20;

const firstPage: GetTenantUsersResponse = await getTenantUsers(tenantId);
const secondPage: GetTenantUsersResponse = await getTenantUsers(tenantId, skip);
[inline-code-end]

---