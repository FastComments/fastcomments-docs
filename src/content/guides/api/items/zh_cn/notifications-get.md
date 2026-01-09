[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

此路由返回最多 30 个 `Notification` 对象，按 `createdAt` 排序，最新的在前。

您可以按 `userId` 过滤。启用 SSO 时，用户 id 的格式为 `<tenant id>:<user id>`。

[inline-code-attrs-start title = '用户未读通知 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = '通知 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** 通过跳过记录进行分页。 **/
    skip?: number
    /** 按用户过滤。 **/
    userId?: string
    /** 按 urlId 过滤。 **/
    urlId?: string
    /** 按源评论过滤。 **/
    fromCommentId?: string
    /** 按已读/未读过滤。 **/
    viewed?: 'true' | 'false'
    /** 按类型过滤。 **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = '通知 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 失败时包含。 **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]