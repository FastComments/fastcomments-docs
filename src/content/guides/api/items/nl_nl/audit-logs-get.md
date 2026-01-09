[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Deze API gebruikt paginering, geleverd door de parameters `skip`, `before` en `after`. AuditLogs worden geretourneerd in pagina's van `1000`, geordend op `when` en `id`.

Het ophalen van elke `1000` logs kost `10` credits.

Standaard ontvangt u een lijst met **de nieuwste items eerst**. Op deze manier kunt u beginnen met pollen met `skip=0`, en pagineren totdat u het laatste record vindt dat u hebt geconsumeerd.

Als alternatief kunt u sorteren op oudste-eerst en pagineren totdat er geen records meer zijn.

Sorteren kan worden gedaan door `order` in te stellen op `ASC` of `DESC`. De standaard is `ASC`.

Zoeken op datum is mogelijk via `before` en `after` als tijdstempels met milliseconden. `before` en `after` zijn NIET inclusief.

[inline-code-attrs-start title = 'AuditLog cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'AuditLog Antwoordstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij een fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Opgenomen bij een fout. **/
    reason?: string
    /** De logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---