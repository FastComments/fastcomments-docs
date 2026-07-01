## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| skip | number | Nee |  |

## Respons

Retourneert: [`GetTenantUsersResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsersResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTenantUsers Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_9f7a2c";
  const skip: number = 25;

  const usersPage: GetTenantUsersResponse1 = await getTenantUsers(tenantId, skip);
  const allUsers: GetTenantUsersResponse1 = await getTenantUsers(tenantId);
})();
[inline-code-end]