[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

此路由提供替換單一個 `TenantUser` 的功能。

替換 `TenantUser` 有以下限制：

- `signUpDate` 不能是未來的日期。
- `locale` 必須位於 [支援的區域設定](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) 列表中。
- `username` 必須在整個 FastComments.com 中唯一。如果這成為問題，我們建議改用 SSO。
- `email` 必須在整個 FastComments.com 中唯一。如果這成為問題，我們建議改用 SSO。
- 不能更新使用者的 `tenantId`。

我們可以按如下方式建立 `TenantUser`

[inline-code-attrs-start title = 'TenantUser 替換 cURL 範例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 替換請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** 當更改 email 或 username 時，您可以將此設為 true 以同時更新使用者的留言。這會使點數成本加倍。 **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 替換回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** 失敗時包含。 **/
    reason?: string
}
[inline-code-end]