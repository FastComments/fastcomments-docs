[api-resource-header-start name = 'PendingWebhookEvent'; route = 'DELETE /api/v1/pending-webhook-events/:id'; creditsCost = 1; api-resource-header-end]

此路由允许删除单个 `PendingWebhookEvent`。

如果您需要批量删除，请调用带分页的 GET API，然后依次调用此 API。

[inline-code-attrs-start title = 'DELETE PendingWebhookEvent cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = '删除 PendingWebhookEvent 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '删除 PendingWebhookEvent 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventDeleteResponse {
    status: 'success' | 'failed'
    /** 仅在失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 仅在失败时包含。 **/
    reason?: string
}
[inline-code-end]

---