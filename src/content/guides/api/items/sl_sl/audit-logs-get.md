[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Ta API uporablja straničenje, ki ga zagotavljata parametra `skip`, `before` in `after`. AuditLogi se vračajo v straneh po `1000`, razvrščeni po `when` in `id`.

Pridobivanje vsakih `1000` zapisov stane `10` kreditov.

Privzeto boste prejeli seznam z **najnovejšimi elementi na vrhu**. Na ta način lahko začnete z `skip=0` in straničite, dokler ne najdete zadnjega zapisa, ki ste ga že obdelali.

Alternativno lahko razvrstite najprej najstarejše in straničite, dokler ne bo več zapisov.

Razvrščanje lahko izvedete tako, da nastavite `order` na `ASC` ali `DESC`. Privzeto je `ASC`.

Poizvedovanje po datumu je mogoče z `before` in `after` kot časovnimi žigi v milisekundah. `before` in `after` nista vključujoča.

[inline-code-attrs-start title = 'Primer cURL zahteve za AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odgovora za AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Vključeno ob napaki. **/
    reason?: string
    /** Dnevniki! **/
    auditLogs: AuditLog[]
}
[inline-code-end]