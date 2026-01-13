[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Diese API verwendet Paginierung, bereitgestellt durch die Parameter `skip`, `before` und `after`. AuditLogs werden in Seiten von `1000` zurückgegeben, sortiert nach `when` und `id`.

Das Abrufen von jeweils `1000` Logs hat Credit-Kosten von `10`.

Standardmäßig erhalten Sie eine Liste mit **den neuesten Elementen zuerst**. So können Sie beginnend mit `skip=0` abfragen und paginieren, bis Sie den letzten verarbeiteten Datensatz finden.

Alternativ können Sie älteste-zuerst sortieren und paginieren, bis keine Datensätze mehr vorhanden sind.

Die Sortierung kann durch Setzen von `order` auf entweder `ASC` oder `DESC` erfolgen. Der Standard ist `ASC`.

Abfragen nach Datum sind über `before` und `after` als Zeitstempel mit Millisekunden möglich. `before` und `after` sind NICHT inklusive.

[inline-code-attrs-start title = 'AuditLog cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Anfrage-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'AuditLog Antwort-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
