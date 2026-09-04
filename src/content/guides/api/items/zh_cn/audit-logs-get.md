[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

此 API 使用分页，由 `skip`、`limit`、`before` 和 `after` 参数提供。AuditLogs 默认以 `1000` 条为一页返回，最大 `limit` 为 `10000`，按 `when` 和 `id` 排序。由于此端点通常用于一次性导出历史记录，而不是交互式分页浏览，页面会比较大。

每返回 `100` 条日志消耗 `1` 积分。

默认情况下，您将收到 **最新的项目在前** 的列表。这样，您可以从 `skip=0` 开始轮询，分页直到找到您已消费的最后一条记录。

或者，您可以按最旧的在前排序，并分页直到没有更多记录。

通过将 `order` 设置为 `ASC` 或 `DESC` 可以进行排序。默认是 `DESC`。

可以使用 `before` 和 `after`（以毫秒为单位的时间戳）按日期查询。`before` 和 `after` 为非包含性，可单独使用其中任意一个。

## 查找某人的操作记录

每个事件记录了执行者（`username`、`userId`、`ip`）以及被操作的对象。`targetLabel` 是该对象的可读标签，例如 `jsmith (jsmith@example.com)`，`targetId` 是其 ID。当您知道某人的姓名或电子邮件但不知道其 ID 时，可使用 `target` 对标签进行大小写不敏感的子字符串匹配。

删除操作会在事件发生时捕获标签，因此即使底层记录已被删除，仍可识别被删除的用户或版主。

## 托管租户

如果您的租户管理其他租户，请将 `includeManagedTenants=true`，以在单个响应中返回您租户及其管理的所有租户的事件。每条返回的日志的 `tenantId` 指示其来源租户。

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
    /** 最大 10000。默认 1000。 **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** 仅限此用户名执行的事件。 **/
    username?: string
    /** 仅限此 IP 地址的事件。 **/
    ip?: string
    /** 仅限此类型的事件。 **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** 仅限此资源的事件，例如用户或版主。 **/
    resourceName?: string
    /** 仅限受影响对象具有此 ID 的事件。 **/
    targetId?: string
    /** 对受影响对象标签的大小写不敏感的子字符串匹配。 **/
    target?: string
    /** 还返回此租户管理的租户的事件。 **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** 失败时包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** 失败时包含。 **/
    reason?: string
    /** 日志！ **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---