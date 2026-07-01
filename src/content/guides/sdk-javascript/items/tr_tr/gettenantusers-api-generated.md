## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|----|-----|----------|----------|
| tenantId | string | Evet |  |
| skip | number | Hayır |  |

## Yanıt

Döndürür: [`GetTenantUsersResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsersResponse1.ts)

## Örnek

[inline-code-attrs-start title = 'getTenantUsers Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_9f7a2c";
  const skip: number = 25;

  const usersPage: GetTenantUsersResponse1 = await getTenantUsers(tenantId, skip);
  const allUsers: GetTenantUsersResponse1 = await getTenantUsers(tenantId);
})();
[inline-code-end]