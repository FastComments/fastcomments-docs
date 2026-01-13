## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |

## Odpowiedź

Zwraca: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f7d4b2a-1c3e";
const id: string = "user_6a12b3c4d5";
const includeProfile: boolean | undefined = true; // przykład opcjonalnego parametru
const response: GetTenantUser200Response = await getTenantUser(tenantId, id);
console.log("Tenant user fetched", response);
[inline-code-end]

---