[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Diese API verwendet Paginierung, bereitgestellt durch die Parameter `skip`, `limit`, `before` und `after`. AuditLogs werden standardmäßig in Seiten von `1000` zurückgegeben, bis zu einer maximalen `limit` von `10000`, sortiert nach `when` und `id`. Die Seiten sind groß, weil dieser Endpunkt normalerweise verwendet wird, um den Verlauf zu exportieren, anstatt interaktiv durch ihn zu blättern.

Jede zurückgegebene Menge von `100` Logs kostet `1` Credit.

Standardmäßig erhalten Sie eine Liste mit **den neuesten Einträgen zuerst**. Auf diese Weise können Sie mit `skip=0` beginnen und paginieren, bis Sie den zuletzt verarbeiteten Datensatz gefunden haben.

Alternativ können Sie aufsteigend sortieren (älteste zuerst) und paginieren, bis keine weiteren Datensätze mehr vorhanden sind.

Die Sortierung kann durch Setzen von `order` auf `ASC` oder `DESC` erfolgen. Der Standardwert ist `DESC`.

Eine Abfrage nach Datum ist über `before` und `after` als Zeitstempel mit Millisekunden möglich. `before` und `after` sind NICHT inklusive, und jeder kann auch allein verwendet werden.

## Herausfinden, was mit einer Person passiert ist

Jedes Ereignis zeichnet auf, wer es ausgeführt hat (`username`, `userId`, `ip`) und, separat, worauf es ausgeführt wurde. `targetLabel` ist ein menschenlesbares Label für dieses Objekt, zum Beispiel `jsmith (jsmith@example.com)`, und `targetId` ist dessen ID. Verwenden Sie `target` für eine Übereinstimmung ohne Groß-/Kleinschreibung als Teilstring auf dem Label, wenn Sie den Namen oder die E-Mail einer Person kennen, aber nicht deren ID.

Löschungen erfassen das Label zum Zeitpunkt des Ereignisses, sodass ein entfernter Benutzer oder Moderator auch dann identifiziert werden kann, wenn der zugrunde liegende Datensatz nicht mehr existiert.

## Verwaltete Tenants

Wenn Ihr Tenant andere Tenants verwaltet, setzen Sie `includeManagedTenants=true`, um Ereignisse von Ihrem Tenant und jedem von ihm verwalteten Tenant in einer Antwort zurückzugeben. Die `tenantId` jedes zurückgegebenen Logs zeigt Ihnen, von welchem Tenant es stammt.

[inline-code-attrs-start title = 'AuditLog cURL-Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'AuditLog Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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