[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

此路由提供更新單一 `TenantUser` 的功能。

更新 `TenantUser` 有下列限制：

- `signUpDate` 不能設定為未來時間。
- `locale` 必須位於 [支援的語系](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) 列表中。
- `username` 必須在整個 FastComments.com 中唯一。如果這成為問題，我們建議改為使用 SSO。
- `email` 必須在整個 FastComments.com 中唯一。如果這成為問題，我們建議改為使用 SSO。
- 您無法更新使用者的 `tenantId`。

我們可以建立一個 `TenantUser` 如下

[inline-code-attrs-start title = 'TenantUser 更新 cURL 範例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 更新請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** 當 email 或 username 被更改時，您可以將此設定為 true 以同時更新該使用者的評論。這會使點數成本加倍。 **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 更新回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** 失敗時包含。 **/
    reason?: string
}
[inline-code-end]

---