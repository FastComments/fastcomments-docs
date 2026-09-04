[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

This API uses pagination, provided by the `skip`, `limit`, `before`, and `after` parameters. AuditLogs are returned in pages of `5000` by default, up to a maximum `limit` of `10000`, ordered by `when` and `id`. The pages are large because this endpoint is usually used to dump history rather than to page through it interactively.

Every `100` logs returned has a credit cost of `1`.

By default, you will receive a list with **the newest items first**. This way, you can poll starting with `skip=0`, paginating until you find the last record you've consumed.

Alternatively, you can sort oldest-first, and paginate until there are no more records.

Sorting can be done by setting `order` to either `ASC` or `DESC`. The default is `DESC`.

Querying by date is possible via `before` and `after` as timestamps with milliseconds. `before` and `after` are NOT inclusive, and either can be used on its own.

## Finding what happened to a person

Every event records who performed it (`username`, `userId`, `ip`) and, separately, what it was performed on. `targetLabel` is a human-readable label for that object, for example `jsmith (jsmith@example.com)`, and `targetId` is its id. Use `target` for a case-insensitive substring match on the label when you know a person's name or email but not their id.

Deletes capture the label at the time of the event, so a removed user or moderator can still be identified after the underlying record is gone.

## Managed tenants

If your tenant manages other tenants, set `includeManagedTenants=true` to return events from your tenant and every tenant it manages in one response. Each returned log's `tenantId` tells you which tenant it came from.

[inline-code-attrs-start title = 'AuditLog cURL Primer'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Struktura Zahteva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Maksimum 10000. Podrazumevano 5000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Samo događaji izvršeni od strane ovog korisničkog imena. **/
    username?: string
    /** Samo događaji sa ove IP adrese. **/
    ip?: string
    /** Samo događaji ovog tipa. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Samo događaji za ovaj resurs, npr. User ili Moderator. **/
    resourceName?: string
    /** Samo događaji čiji je pogođeni objekat ima ovaj ID. **/
    targetId?: string
    /** Podudaranje podstringa neosetljivog na veličinu slova u oznaci pogođenog objekta. **/
    target?: string
    /** Takođe vrati događaje iz tenant-a koje ovaj tenant upravlja. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Struktura Odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Uključeno u slučaju greške. **/
    reason?: string
    /** Logovi! **/
    auditLogs: AuditLog[]
}
[inline-code-end]