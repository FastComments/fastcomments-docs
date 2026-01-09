[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

此路由提供新增單一 `TenantUser` 的功能。

建立 `TenantUser` 有下列限制：

- `username` 為必填。
- `email` 為必填。
- `signUpDate` 不能是未來的時間。
- `locale` 必須位於 [支援的語系](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) 列表中。
- `username` 必須在整個 FastComments.com 中唯一。如果這成為問題，我們建議改用 SSO。
- `email` 必須在整個 FastComments.com 中唯一。如果這成為問題，我們建議改用 SSO。
- 您不得建立超過套件中 `maxTenantUsers` 定義的租戶使用者數量。

我們可依下列方式建立 `TenantUser`

[inline-code-attrs-start title = 'TenantUser 建立 cURL 範例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 建立請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 建立回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** 失敗時會包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** 失敗時會包含。 **/
    reason?: string
    tenantUser?: TenantUser; // 成功時回傳完整建立的租戶使用者。
}
[inline-code-end]

---