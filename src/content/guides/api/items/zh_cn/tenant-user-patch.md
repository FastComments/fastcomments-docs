[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

此路由提供更新单个 `TenantUser` 的功能。

更新 `TenantUser` 有以下限制：

- `signUpDate` 不能是将来的日期。
- `locale` 必须在 [Supported Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) 列表中。
- `username` 必须在整个 FastComments.com 中唯一。如果有冲突，建议改用 SSO。
- `email` 必须在整个 FastComments.com 中唯一。如果有冲突，建议改用 SSO。
- 不能更新用户的 `tenantId`。

我们可以按如下方式创建 `TenantUser`

[inline-code-attrs-start title = 'TenantUser 更新 cURL 示例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 更新请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** 当更改 email 或 username 时，可以将此设置为 true 以同时更新用户的评论。这将使信用成本加倍。 **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 更新响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** 在失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** 在失败时包含。 **/
    reason?: string
}
[inline-code-end]