## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createTenantUserBody | CreateTenantUserBody | Evet |  |

## Yanıt

Döndürür: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Örnek

[inline-code-attrs-start title = 'createTenantUser Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f4a2b";
const createTenantUserBody: CreateTenantUserBody = {
  email: "jane.doe@example.com",
  firstName: "Jane",
  lastName: "Doe",
  role: "commenter",
  approved: true,
  displayName: "Jane D." // isteğe bağlı: kullanıcı dostu bir ad sağlama
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
console.log(result);
[inline-code-end]