[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

此 API 使用分页，由 `skip`、`before` 和 `after` 参数提供。AuditLogs 以每页 `1000` 的形式返回，按 `when` 和 `id` 排序。

获取每 `1000` 条日志将消耗 `10` 积分。

默认情况下，您将收到一个 **按最新项优先** 的列表。这样，您可以从 `skip=0` 开始轮询，分页直到找到您已消费的最后一条记录。

或者，您可以按最旧项优先排序，然后分页直到没有更多记录。

可以通过将 `order` 设置为 `ASC` 或 `DESC` 来进行排序。默认值为 `ASC`。

可以通过 `before` 和 `after` 使用带毫秒的时间戳按日期查询。`before` 和 `after` 不包含边界值。

[inline-code-attrs-start title = 'AuditLog cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失败时包含。 **/
    reason?: string
    /** 日志！ **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---