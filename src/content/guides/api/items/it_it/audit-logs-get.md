[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Questa API utilizza la paginazione, fornita dai parametri `skip`, `limit`, `before` e `after`. I AuditLogs vengono restituiti in pagine di `1000` per impostazione predefinita, fino a un `limit` massimo di `10000`, ordinati per `when` e `id`. Le pagine sono grandi perché questo endpoint è solitamente usato per scaricare la cronologia piuttosto che per scorrere interattivamente.

Ogni `100` log restituiti hanno un costo di credito di `1`.

Per impostazione predefinita, riceverai un elenco con **gli elementi più recenti per primi**. In questo modo, puoi effettuare il polling iniziando con `skip=0`, paginando fino a trovare l'ultimo record consumato.

In alternativa, puoi ordinare dal più vecchio al più recente e paginare finché non ci sono più record.

L'ordinamento può essere effettuato impostando `order` su `ASC` o `DESC`. Il valore predefinito è `DESC`.

È possibile interrogare per data tramite `before` e `after` come timestamp in millisecondi. `before` e `after` NON sono inclusivi e ciascuno può essere usato da solo.

## Finding what happened to a person

## Trovare cosa è successo a una persona

Every event records who performed it (`username`, `userId`, `ip`) and, separately, what it was performed on. `targetLabel` is a human-readable label for that object, for example `jsmith (jsmith@example.com)`, and `targetId` is its id. Use `target` for a case-insensitive substring match on the label when you know a person's name or email but not their id.

Ogni evento registra chi lo ha eseguito (`username`, `userId`, `ip`) e, separatamente, su cosa è stato eseguito. `targetLabel` è un'etichetta leggibile dall'uomo per quell'oggetto, ad esempio `jsmith (jsmith@example.com)`, e `targetId` è il suo ID. Usa `target` per una corrispondenza di sottostringa case‑insensitive sull'etichetta quando conosci il nome o l'email di una persona ma non il suo ID.

Deletes capture the label at the time of the event, so a removed user or moderator can still be identified after the underlying record is gone.

Le cancellazioni catturano l'etichetta al momento dell'evento, quindi un utente o moderatore rimosso può ancora essere identificato dopo che il record sottostante è stato eliminato.

## Managed tenants

## Tenant gestiti

If your tenant manages other tenants, set `includeManagedTenants=true` to return events from your tenant and every tenant it manages in one response. Each returned log's `tenantId` tells you which tenant it came from.

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
    /** Max 10000. Predefinito a 1000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Solo gli eventi eseguiti da questo username. **/
    username?: string
    /** Solo gli eventi da questo indirizzo IP. **/
    ip?: string
    /** Solo gli eventi di questo tipo. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Solo gli eventi per questa risorsa, ad es. Utente o Moderatore. **/
    resourceName?: string
    /** Solo gli eventi il cui oggetto interessato ha questo ID. **/
    targetId?: string
    /** Corrispondenza di sottostringa case‑insensitive sull'etichetta dell'oggetto interessato. **/
    target?: string
    /** Restituisce anche gli eventi dai tenant che questo tenant gestisce. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Incluso in caso di errore. **/
    reason?: string
    /** I log! **/
    auditLogs: AuditLog[]
}
[inline-code-end]