## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | number | Nee |  |

## Response

Retourneert: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsers200Response.ts)

## Example

[inline-code-attrs-start title = 'getTenantUsers Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_8a3f2c';
const skip: number = 50;
const usersWithSkip: GetTenantUsers200Response = await getTenantUsers(tenantId, skip);
const usersNoSkip: GetTenantUsers200Response = await getTenantUsers(tenantId);
[inline-code-end]

---