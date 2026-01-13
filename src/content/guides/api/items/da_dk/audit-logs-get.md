[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 1; api-resource-header-end]

Denne rute returnerer audit logs i sider af `100`. Paginering leveres af `skip`-parameteren. Audit logs er sorteret efter `createdAt` og `id`.

[inline-code-attrs-start title = 'AuditLog cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    skip?: number
    /** Filter by creator (user id). **/
    createdBy?: string
    /** A start date to filter. Inclusive. Should be JSON (ex 2021-01-01T00:00:00.000Z) **/
    startDate?: string
    /** An end date to filter. Exclusive. Should be JSON (ex 2021-01-01T00:00:00.000Z) **/
    endDate?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    auditLogs: AuditLog[]
}
[inline-code-end]
