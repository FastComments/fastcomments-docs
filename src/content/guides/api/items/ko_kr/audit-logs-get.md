[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

This API uses pagination, provided by the `skip`, `limit`, `before`, and `after` parameters. AuditLogs are returned in pages of `1000` by default, up to a maximum `limit` of `10000`, ordered by `when` and `id`. The pages are large because this endpoint is usually used to dump history rather than to page through it interactively.

Every `100` logs returned has a credit cost of `1`.

By default, you will receive a list with **the newest items first**. This way, you can poll starting with `skip=0`, paginating until you find the last record you've consumed.

Alternatively, you can sort oldest-first, and paginate until there are no more records.

Sorting can be done by setting `order` to either `ASC` or `DESC`. The default is `DESC`.

Querying by date is possible via `before` and `after` as timestamps with milliseconds. `before` and `after` are NOT inclusive, and either can be used on its own.

## 사람에게 무슨 일이 있었는지 찾기

Every event records who performed it (`username`, `userId`, `ip`) and, separately, what it was performed on. `targetLabel` is a human-readable label for that object, for example `jsmith (jsmith@example.com)`, and `targetId` is its id. Use `target` for a case-insensitive substring match on the label when you know a person's name or email but not their id.

Deletes capture the label at the time of the event, so a removed user or moderator can still be identified after the underlying record is gone.

## 관리되는 테넌트

If your tenant manages other tenants, set `includeManagedTenants=true` to return events from your tenant and every tenant it manages in one response. Each returned log's `tenantId` tells you which tenant it came from.

[inline-code-attrs-start title = 'AuditLog cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Max 10000. Defaults to 1000. **/
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

[inline-code-attrs-start title = 'AuditLog 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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