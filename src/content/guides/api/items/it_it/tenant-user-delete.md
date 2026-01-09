[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Questa route consente la rimozione di un `TenantUser` tramite ID.

È possibile eliminare i commenti dell'utente tramite il parametro di query `deleteComments`. Nota che se questo è true:

1. Tutti i commenti dell'utente saranno eliminati in tempo reale.
2. Tutti i __child__ (ora orfani) commenti saranno eliminati o anonimizzati in base alla configurazione della pagina associata a ciascun commento. Ad esempio se la modalità di cancellazione del thread è "anonymize", allora le risposte rimarranno, e i commenti dell'utente saranno anonimizzati. Questo si applica solo quando `commentDeleteMode` è `Remove` (il valore predefinito).
3. Il `creditsCost` diventa `2`.

### Commenti anonimizzati

Puoi conservare i commenti dell'utente ma semplicemente anonimizzarli impostando `commentDeleteMode=1`.

Se i commenti dell'utente vengono anonimizzati, i seguenti valori vengono impostati su null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` e `isDeletedUser` vengono impostati a `true`.

Durante il rendering, il widget dei commenti utilizzerà `DELETED_USER_PLACEHOLDER` (default: "[deleted]") per il nome dell'utente e `DELETED_CONTENT_PLACEHOLDER` per il commento. Questi possono essere personalizzati tramite l'interfaccia di personalizzazione del Widget.

### Esempi

[inline-code-attrs-start title = 'Esempio cURL per la rimozione di TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta per la rimozione di TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // predefinito
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Puoi impostare questo su true per eliminare anche i commenti dell'utente. Questo raddoppierà il costo in crediti. **/
    deleteComments?: 'true' | 'false'
    /** Puoi impostarlo come desideri per determinare come gestire i commenti dell'utente. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta per la rimozione di TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Incluso in caso di errore. **/
    reason?: string
}
[inline-code-end]