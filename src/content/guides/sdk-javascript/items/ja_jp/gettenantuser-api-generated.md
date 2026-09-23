## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## レスポンス

返却: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUserResponse.ts)

## 例

[inline-code-attrs-start title = 'getTenantUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-tenant-001";
    const userId: string = "user-12345";
    const response: GetTenantUserResponse = await getTenantUser(tenantId, userId);
    console.log(response);
})();
[inline-code-end]