[api-resource-header-start name = 'SSOUser'; route = 'GET /api/v1/sso-users/by-email/:email'; creditsCost = 1; api-resource-header-end]

此路由會依使用者的電子郵件回傳單一 SSO 使用者。

[inline-code-attrs-start title = '依電子郵件取得 SSO 使用者 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/sso-users/by-email/someone@somewhere.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserRequestByEmailQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserByEmailResponse {
    status: 'success' | 'failed'
    /** 僅於失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-email' | 'user-does-not-exist'
    /** 僅於失敗時包含。 **/
    reason?: string
    user: SSOUser
}
[inline-code-end]

---