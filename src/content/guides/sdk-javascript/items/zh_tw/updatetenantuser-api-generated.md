## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateTenantUserBody | UpdateTenantUserBody | 是 |  |
| updateComments | string | 否 |  |

## 回應

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 範例

[inline-code-attrs-start title = 'updateTenantUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---