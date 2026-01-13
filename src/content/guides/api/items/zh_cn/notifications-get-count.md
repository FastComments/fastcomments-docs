[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

此路由返回一个对象，该对象在 `count` 参数下包含通知数量。

它比 `/notification-count/` 慢且消耗双倍积分，但允许在更多维度上进行过滤。

您可以按与 `/notifications` 端点相同的参数进行过滤，例如 `userId`。使用 SSO 时，用户 ID 的格式为 `<tenant id>:<user id>`。

[inline-code-attrs-start title = '用户未读通知计数 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = '用于特定页面的用户未读通知计数 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '通知计数请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** 按用户过滤。 **/
    userId?: string
    /** 按 urlId 过滤。 **/
    urlId?: string
    /** 按来源评论过滤。 **/
    fromCommentId?: string
    /** 按已读/未读过滤。 **/
    viewed?: 'true' | 'false'
    /** 按类型过滤。 **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = '通知计数响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 失败时包含。 **/
    reason?: string
    count?: number
}
[inline-code-end]