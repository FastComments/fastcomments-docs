## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | string | No |  |
| commentDeleteMode | string | No |  |

## 回應

返回：[`DeleteTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantUserResponse.ts)

## 範例

[inline-code-attrs-start title = 'deleteTenantUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoDeleteTenantUser() {
  const tenantId: string = "acme-corp-tenant";
  const userId: string = "user-9876";

  // 刪除使用者及其所有評論，使用硬刪除模式
  const resultWithOptions: DeleteTenantUserResponse = await deleteTenantUser(
    tenantId,
    userId,
    "true",
    "hard"
  );

  // 刪除使用者但不移除評論（預設行為）
  const resultBasic: DeleteTenantUserResponse = await deleteTenantUser(tenantId, userId);
}
[inline-code-end]