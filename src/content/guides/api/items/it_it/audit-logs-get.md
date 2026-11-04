[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Questa API utilizza la paginazione, fornita dai parametri `skip`, `limit`, `before` e `after`. Gli AuditLogs vengono restituiti in pagine di `1000` per impostazione predefinita, fino a un `limit` massimo di `10000`, ordinati per `when` e `id`. Le pagine sono grandi perché questo endpoint viene in genere utilizzato per esportare la cronologia piuttosto che per sfogliarla interattivamente.

Ogni `100` log restituiti ha un costo di `1` credito.

Per impostazione predefinita, riceverai un elenco con **gli elementi più recenti per primi**. In questo modo, puoi effettuare il polling partendo da `skip=0`, paginando fino a trovare l'ultimo record che hai consumato.

In alternativa, puoi ordinare dal più vecchio al più recente, e paginare finché non ci sono più record.

L'ordinamento può essere effettuato impostando `order` su `ASC` o `DESC`. Il valore predefinito è `DESC`.

L'interrogazione per data è possibile tramite `before` e `after` come timestamp con millisecondi. `before` e `after` NON sono inclusivi, e ciascuno può essere utilizzato da solo.

## Trovare cosa è successo a una persona

Ogni evento registra chi lo ha eseguito (`username`, `userId`, `ip`) e, separatamente, su cosa è stato eseguito. `targetLabel` è un'etichetta leggibile dall'utente per quell'oggetto, ad esempio `jsmith (jsmith@example.com)`, e `targetId` è il suo id. Usa `target` per una corrispondenza di sottostringa senza distinzione tra maiuscole e minuscole sull'etichetta quando conosci il nome o l'email di una persona ma non il suo id.

Le eliminazioni acquisiscono l'etichetta al momento dell'evento, quindi un utente o un moderatore rimosso può ancora essere identificato dopo che il record sottostante è stato eliminato.

## Tenant gestiti

Se il tuo tenant gestisce altri tenant, imposta `includeManagedTenants=true` per restituire gli eventi del tuo tenant e di ogni tenant che gestisce in un'unica risposta. Il `tenantId` di ogni log restituito ti indica da quale tenant proviene.

[inline-code-attrs-start title = 'Esempio cURL di AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Massimo 10000. Predefinito 1000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Solo eventi eseguiti da questo username. **/
    username?: string
    /** Solo eventi da questo indirizzo IP. **/
    ip?: string
    /** Solo eventi di questo tipo. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Solo eventi per questa risorsa, ad es. User o Moderator. **/
    resourceName?: string
    /** Solo eventi il cui oggetto interessato ha questo id. **/
    targetId?: string
    /** Corrispondenza di sottostringa senza distinzione tra maiuscole e minuscole sull'etichetta dell'oggetto interessato. **/
    target?: string
    /** Restituisce anche gli eventi dai tenant gestiti da questo tenant. **/
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