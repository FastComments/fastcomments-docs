[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

此路由提供向单个 `TenantUser` 发送登录链接的功能。

适用于批量创建用户且不需要向他们说明如何登录到 FastComments.com 的情况。 这将向他们发送一个用于登录的“魔术链接”，该链接在 `30 days` 后过期。

发送登录链接到 `TenantUser` 时存在以下限制：
- `TenantUser` 必须已存在。
- 您必须有权限管理该 `TenantUser` 所属的 `Tenant`。

我们可以按如下方式向 `TenantUser` 发送登录链接：

[inline-code-attrs-start title = 'TenantUser 登录链接 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

这将发送类似 `Bob at TenantName is inviting you to be a moderator...` 的电子邮件

[inline-code-attrs-start title = 'TenantUser 登录链接 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 登录链接 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** 失败时包含。 **/
    reason?: string
}
[inline-code-end]