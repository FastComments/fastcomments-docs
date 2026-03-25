## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Respons

Geeft terug: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTenantUser Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_fc5a9b2c';
const userId: string = 'user_0a12b3';
const result: GetTenantUser200Response = await getTenantUser(tenantId, userId);
const user: User | undefined = (result as any).user; // toegang tot payload
const userEmail: string | undefined = user?.email;
console.log('Fetched user email:', userEmail);
[inline-code-end]