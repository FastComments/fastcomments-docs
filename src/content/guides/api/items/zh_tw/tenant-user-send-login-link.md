[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

此路由提供向單一 `TenantUser` 發送登入連結的功能。

在批量建立使用者且不需指示他們如何登入 FastComments.com 時非常有用。此操作會向他們發送一個可於 `30 days` 後過期的 "magic link" 用以登入。

發送登入連結給 `TenantUser` 時有下列限制：
- The `TenantUser` must already exist.
- 您必須能管理該 `TenantUser` 所屬的 `Tenant`。

我們可以依下列方式向 `TenantUser` 發送登入連結：

[inline-code-attrs-start title = 'TenantUser 登入連結 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

這將會寄出像 `Bob at TenantName is inviting you to be a moderator...` 的電子郵件

[inline-code-attrs-start title = 'TenantUser 登入連結請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 登入連結回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** 在失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** 在失敗時包含。 **/
    reason?: string
}
[inline-code-end]

---