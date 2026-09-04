[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Diese API verwendet Pagination, bereitgestellt durch die Parameter `skip`, `limit`, `before` und `after`. AuditLogs werden standardmäßig in Seiten zu `1000` Einträgen zurückgegeben, bis zu einem maximalen `limit` von `10000`, sortiert nach `when` und `id`. Die Seiten sind groß, weil dieser Endpunkt normalerweise verwendet wird, um die Historie zu exportieren, anstatt interaktiv durch sie zu blättern.

Jede `100` zurückgegebenen Protokolle kosten `1` Credit.

Standardmäßig erhalten Sie eine Liste mit **den neuesten Elementen zuerst**. Auf diese Weise können Sie mit `skip=0` abfragen und paginieren, bis Sie den letzten von Ihnen konsumierten Datensatz finden.

Alternativ können Sie nach dem ältesten zuerst sortieren und paginieren, bis keine weiteren Datensätze mehr vorhanden sind.

Sortierung kann durch Setzen von `order` auf `ASC` oder `DESC` erfolgen. Der Standardwert ist `DESC`.

Abfragen nach Datum sind über `before` und `after` als Zeitstempel mit Millisekunden möglich. `before` und `after` sind NICHT inklusiv und können jeweils einzeln verwendet werden.

## Finding what happened to a person

Jedes Ereignis zeichnet auf, wer es ausgeführt hat (`username`, `userId`, `ip`) und, getrennt davon, worauf es ausgeführt wurde. `targetLabel` ist ein menschenlesbares Label für dieses Objekt, zum Beispiel `jsmith (jsmith@example.com)`, und `targetId` ist dessen ID. Verwenden Sie `target` für eine Groß-/Kleinschreibung ignorierende Teilzeichenketten‑Suche im Label, wenn Sie den Namen oder die E‑Mail einer Person kennen, aber nicht deren ID.

Löschungen erfassen das Label zum Zeitpunkt des Ereignisses, sodass ein gelöschter Benutzer oder Moderator auch nach dem Entfernen des zugrunde liegenden Datensatzes noch identifiziert werden kann.

## Managed tenants

Wenn Ihr Mandant andere Mandanten verwaltet, setzen Sie `includeManagedTenants=true`, um Ereignisse von Ihrem Mandanten und allen von ihm verwalteten Mandanten in einer Antwort zurückzugeben. Die `tenantId` jedes zurückgegebenen Protokolls gibt an, von welchem Mandanten es stammt.

[inline-code-attrs-start title = 'AuditLog cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** Max 10000. Standardwert ist 1000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Nur Ereignisse, die von diesem Benutzernamen durchgeführt wurden. **/
    username?: string
    /** Nur Ereignisse von dieser IP-Adresse. **/
    ip?: string
    /** Nur Ereignisse dieses Typs. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Nur Ereignisse für diese Ressource, z. B. Benutzer oder Moderator. **/
    resourceName?: string
    /** Nur Ereignisse, bei denen das betroffene Objekt diese ID hat. **/
    targetId?: string
    /** Fallunabhängige Teilzeichenketten‑Suche im Label des betroffenen Objekts. **/
    target?: string
    /** Zusätzlich Ereignisse von Mandanten zurückgeben, die dieser Mandant verwaltet. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Bei Fehler enthalten. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Bei Fehler enthalten. **/
    reason?: string
    /** Die Protokolle! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---