[api-resource-header-start name = 'SSOUser'; route = 'GET /api/v1/sso-users'; creditsCost = 10; api-resource-header-end]

此路由會以每頁 `100` 名的分頁方式回傳 SSO 使用者。分頁由 `skip` 參數提供。使用者依其 `signUpDate` 與 `id` 排序。

[inline-code-attrs-start title = 'SSOUsers cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUsers 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUsers 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** 僅在失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 僅在失敗時包含。 **/
    reason?: string
    users: SSOUser[]
}
[inline-code-end]

---