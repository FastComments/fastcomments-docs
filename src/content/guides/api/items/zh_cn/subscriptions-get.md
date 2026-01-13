[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

此路由返回最多 30 个按 `createdAt` 排序的 `Subscription` 对象，最新的在前。

您可以按 `userId` 过滤。使用 SSO 时，用户 ID 的格式为 `<tenant id>:<user id>`。

[inline-code-attrs-start title = '用户订阅 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = '订阅 请求 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** 通过跳过记录进行分页。 **/
    skip?: number
    /** 按用户过滤。 **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = '订阅 响应 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 失败时包含。 **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]