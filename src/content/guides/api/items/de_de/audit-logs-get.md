[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Diese API verwendet Pagination, bereitgestellt durch die Parameter `skip`, `limit`, `before` und `after`. AuditLogs werden standardmäßig in Seiten von `100` zurückgegeben, bis zu einem maximalen `limit` von `200`, sortiert nach `when` und `id`.

Jede zurückgegebene Gruppe von `100` Logs kostet `1` Credit.

Standardmäßig erhalten Sie eine Liste mit **den neuesten Elementen zuerst**. Auf diese Weise können Sie mit `skip=0` abfragen und paginieren, bis Sie den letzten von Ihnen konsumierten Datensatz finden.

Alternativ können Sie nach dem ältesten zuerst sortieren und paginieren, bis keine weiteren Datensätze mehr vorhanden sind.

Die Sortierung kann durch Setzen von `order` auf `ASC` oder `DESC` erfolgen. Der Standardwert ist `DESC`.

Abfragen nach Datum sind über `before` und `after` als Zeitstempel mit Millisekunden möglich. `before` und `after` sind NICHT inklusiv und können jeweils einzeln verwendet werden.

## Finden, was einer Person passiert ist

Jedes Ereignis zeichnet auf, wer es durchgeführt hat (`username`, `userId`, `ip`) und, getrennt davon, worauf es angewendet wurde. `targetLabel` ist eine menschenlesbare Bezeichnung für dieses Objekt, zum Beispiel `jsmith (jsmith@example.com)`, und `targetId` ist dessen ID. Verwenden Sie `target` für eine case‑insensitive Teilstring‑Suche nach der Bezeichnung, wenn Sie den Namen oder die E‑Mail einer Person kennen, aber nicht deren ID.

Löschvorgänge erfassen die Bezeichnung zum Zeitpunkt des Ereignisses, sodass ein gelöschter Benutzer oder Moderator auch nach dem Entfernen des zugrunde liegenden Datensatzes noch identifiziert werden kann.

## Verwaltete Mandanten

Wenn Ihr Mandant andere Mandanten verwaltet, setzen Sie `includeManagedTenants=true`, um Ereignisse sowohl von Ihrem Mandanten als auch von allen von ihm verwalteten Mandanten in einer Antwort zurückzugeben. Die `tenantId` jedes zurückgegebenen Logs gibt an, von welchem Mandanten es stammt.

[inline-code-attrs-start title = 'AuditLog cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
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
    /** Case‑insensitive Teilstring‑Suche im Label des betroffenen Objekts. **/
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
    /** Die Logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]