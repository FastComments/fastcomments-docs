[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

此 API 使用分页，由 `skip`、`limit`、`before` 和 `after` 参数提供。AuditLogs 默认以 `5000` 条为一页返回，最大 `limit` 为 `10000`，按 `when` 和 `id` 排序。页面较大是因为此端点通常用于一次性导出历史记录，而不是交互式分页浏览。

每返回 `100` 条日志会消耗 `1` 积分。

默认情况下，您将收到 **最新的项目在前** 的列表。这样，您可以从 `skip=0` 开始轮询，分页直到找到您已消费的最后一条记录。

或者，您可以按最旧的在前排序，并分页直到没有更多记录。

可以通过将 `order` 设置为 `ASC` 或 `DESC` 来进行排序。默认是 `DESC`。

可以使用毫秒时间戳的 `before` 和 `after` 参数按日期查询。`before` 和 `after` **不包含** 边界值，且任意一个都可以单独使用。

## 查找某人的操作记录

每个事件记录了执行者（`username`、`userId`、`ip`），以及被执行的对象。`targetLabel` 是该对象的可读标签，例如 `jsmith (jsmith@example.com)`，`targetId` 是其 ID。当您知道某人的姓名或电子邮件但不知道其 ID 时，可使用 `target` 对标签进行不区分大小写的子字符串匹配。

删除操作会在事件发生时捕获标签，因此即使底层记录已被删除，仍然可以识别被删除的用户或版主。

## 托管租户

如果您的租户管理其他租户，请将 `includeManagedTenants=true`，以在一次响应中返回您租户及其管理的所有租户的事件。每条返回的日志的 `tenantId` 指示其来源租户。

[inline-code-attrs-start title = 'AuditLog cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Max 10000. Defaults to 5000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Only events performed by this username. **/
    username?: string
    /** Only events from this IP address. **/
    ip?: string
    /** Only events of this type. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Only events for this resource, e.g. User or Moderator. **/
    resourceName?: string
    /** Only events whose affected object has this id. **/
    targetId?: string
    /** Case-insensitive substring match on the affected object's label. **/
    target?: string
    /** Also return events from tenants this tenant manages. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]