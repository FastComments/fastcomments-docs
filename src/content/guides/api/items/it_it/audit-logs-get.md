[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Questa API utilizza la paginazione, fornita dai parametri `skip`, `before` e `after`. AuditLogs vengono restituiti in pagine di `1000`, ordinate per `when` e `id`.

Recuperare ogni `1000` log ha un costo in crediti di `10`.

Per impostazione predefinita, riceverai una lista con **i più recenti per primi**. In questo modo, puoi eseguire il polling iniziando con `skip=0`, impaginando finché non trovi l'ultimo record che hai consumato.

In alternativa, puoi ordinare dal più vecchio al più recente e impaginare fino a quando non ci sono più record.

L'ordinamento può essere effettuato impostando `order` su `ASC` o `DESC`. Il valore predefinito è `ASC`.

È possibile effettuare query per data tramite `before` e `after` come timestamp in millisecondi. `before` e `after` NON sono inclusivi.

[inline-code-attrs-start title = 'Esempio cURL di AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struttura della risposta AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluso in caso di errore. **/
    reason?: string
    /** I log! **/
    auditLogs: AuditLog[]
}
[inline-code-end]