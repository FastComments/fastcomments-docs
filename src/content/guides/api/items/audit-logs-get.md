[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

This API uses pagination, provided by the `skip`, `before`, and `after` parameters. AuditLogs are returned in pages of `1000`, ordered by `when` and `id`.

Fetching every `1000` logs has a credit cost of `10`.

By default, you will receive a list with **the newest items first**. This way, you can poll starting with `skip=0`, paginating until you find the last record you've consumed.

Alternatively, you can sort oldest-first, and paginate until there are no more records.

Sorting can be done by setting `order` to either `ASC` or `DESC`. The default is `ASC`.

Querying by date is possible via `before` and `after` as timestamps with milliseconds.

[inline-code-attrs-start title = 'AuditLog cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'AuditLog Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]
