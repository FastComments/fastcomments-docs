[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

此路由可用于替换单个 `TenantUser`。

替换 `TenantUser` 有以下限制：

- `signUpDate` 不得设置为未来时间。
- `locale` 必须在 [支持的语言](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) 列表中。
- `username` 必须在 FastComments.com 全站唯一。如果出现冲突，建议改用 SSO。
- `email` 必须在 FastComments.com 全站唯一。如果出现冲突，建议改用 SSO。
- 无法更新用户的 `tenantId`。

我们可以按如下方式创建 `TenantUser`

[inline-code-attrs-start title = 'TenantUser 替换 cURL 示例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 替换 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** 当 email 或 username 更改时，可以将此设置为 true 以同时更新该用户的评论。 这会使计费消耗翻倍。 **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 替换 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** 失败时包含。 **/
    reason?: string
}
[inline-code-end]

---