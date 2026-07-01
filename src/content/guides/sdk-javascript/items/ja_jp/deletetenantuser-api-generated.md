## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | string | No |  |
| commentDeleteMode | string | No |  |

## レスポンス

戻り値: [`DeleteTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteTenantUserResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteTenantUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoDeleteTenantUser() {
  const tenantId: string = "acme-corp-tenant";
  const userId: string = "user-9876";

  // ユーザーとそのすべてのコメントを、ハード削除モードで削除します
  const resultWithOptions: DeleteTenantUserResponse = await deleteTenantUser(
    tenantId,
    userId,
    "true",
    "hard"
  );

  // コメントを削除せずにユーザーを削除します（デフォルトの動作）
  const resultBasic: DeleteTenantUserResponse = await deleteTenantUser(tenantId, userId);
}
[inline-code-end]