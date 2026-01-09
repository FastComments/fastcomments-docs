[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

此路由提供添加单个 `TenantUser` 的功能。

创建 `TenantUser` 有以下限制：

- `username` 是必需的。
- `email` 是必需的。
- `signUpDate` 不能是未来的日期。
- `locale` 必须位于 [支持的语言](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) 列表中。
- `username` 必须在整个 FastComments.com 上唯一。如果这是个问题，我们建议改用 SSO。
- `email` 必须在整个 FastComments.com 上唯一。如果这是个问题，我们建议改用 SSO。
- 您不得创建超过您套餐中 `maxTenantUsers` 定义数量的租户用户。 

我们可以按如下方式创建 `TenantUser`

[inline-code-attrs-start title = 'TenantUser 创建 cURL 示例'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 创建请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser 创建响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** 失败时包含。 **/
    reason?: string
    tenantUser?: TenantUser; // 在成功时返回完整的已创建租户用户。
}
[inline-code-end]