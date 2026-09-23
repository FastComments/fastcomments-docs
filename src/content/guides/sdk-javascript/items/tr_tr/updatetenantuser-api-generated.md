## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateTenantUserBody | UpdateTenantUserBody | Evet |  |
| updateComments | string | Hayır |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Örnek

[inline-code-attrs-start title = 'updateTenantUser Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runUpdate() {
  const tenantId: string = "tenant_12345";
  const userId: string = "user_987";
  const updateBody: UpdateTenantUserBody = {
    email: "new.email@example.com",
    role: "admin",
    isActive: true
  };
  const comment: string = "Promoted to admin role";

  const result: APIEmptyResponse = await updateTenantUser(tenantId, userId, updateBody, comment);
  console.log(result);
}

runUpdate();
[inline-code-end]