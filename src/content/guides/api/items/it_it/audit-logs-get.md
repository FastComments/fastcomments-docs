[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Questa API utilizza la paginazione, fornita dai parametri `skip`, `limit`, `before` e `after`. I AuditLog vengono restituiti in pagine di `100` per impostazione predefinita, fino a un `limit` massimo di `200`, ordinati per `when` e `id`.

Ogni `100` log restituiti hanno un costo di credito di `1`.

Per impostazione predefinita, riceverai un elenco con **gli elementi più recenti per primi**. In questo modo, puoi effettuare il polling iniziando con `skip=0`, paginando fino a trovare l'ultimo record consumato.

In alternativa, puoi ordinare dal più vecchio al più recente e paginare fino a quando non ci sono più record.

L'ordinamento può essere effettuato impostando `order` su `ASC` o `DESC`. Il valore predefinito è `DESC`.

È possibile interrogare per data tramite `before` e `after` come timestamp in millisecondi. `before` e `after` NON sono inclusivi e ciascuno può essere usato da solo.

## Trovare cosa è successo a una persona

Ogni evento registra chi lo ha eseguito (`username`, `userId`, `ip`) e, separatamente, su cosa è stato eseguito. `targetLabel` è un'etichetta leggibile dall'uomo per quell'oggetto, ad esempio `jsmith (jsmith@example.com)`, e `targetId` è il suo ID. Usa `target` per una corrispondenza di sottostringa non sensibile al maiuscolo/minuscolo sull'etichetta quando conosci il nome o l'email di una persona ma non il suo ID.

Le cancellazioni catturano l'etichetta al momento dell'evento, così un utente o moderatore rimosso può ancora essere identificato dopo che il record sottostante è stato eliminato.

## Tenant gestiti

Se il tuo tenant gestisce altri tenant, imposta `includeManagedTenants=true` per restituire gli eventi dal tuo tenant e da tutti i tenant che gestisce in una singola risposta. Il `tenantId` di ogni log restituito indica da quale tenant proviene.

[inline-code-attrs-start title = 'Esempio cURL AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
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

[inline-code-attrs-start title = 'Struttura della Risposta AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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