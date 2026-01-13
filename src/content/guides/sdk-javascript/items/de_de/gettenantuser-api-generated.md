## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Gibt zurück: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getTenantUser Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f7d4b2a-1c3e";
const id: string = "user_6a12b3c4d5";
const includeProfile: boolean | undefined = true; // Beispiel für optionalen Parameter
const response: GetTenantUser200Response = await getTenantUser(tenantId, id);
console.log("Tenant user fetched", response);
[inline-code-end]

---