---
[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Questa route fornisce la rimozione di un singolo utente SSO tramite il suo id.

Nota che ricaricando il widget dei commenti con un payload per questo utente verrà semplicemente ricreato l'utente in modo trasparente.

La cancellazione dei commenti dell'utente è possibile tramite il parametro di query `deleteComments`. Nota che se questo è true:

1. Tutti i commenti dell'utente saranno cancellati in tempo reale.
2. Tutti i commenti __child__ (ora orfani) saranno cancellati o anonimizzati in base alla configurazione della pagina associata a ciascun commento. Per esempio, se la modalità di cancellazione del thread è "anonymize", allora le risposte rimarranno e i commenti dell'utente verranno anonimizzati. Questo si applica solo quando `commentDeleteMode` è `Remove` (il valore predefinito).
3. Il `creditsCost` diventa `2`.

### Commenti anonimizzati

Puoi conservare i commenti dell'utente ma semplicemente anonimizzarli impostando `commentDeleteMode=1`.

Se i commenti dell'utente vengono anonimizzati, i seguenti valori vengono impostati a null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` e `isDeletedUser` vengono impostati a `true`.

Durante il rendering, il widget dei commenti utilizzerà `DELETED_USER_PLACEHOLDER` (default: "[deleted]") per il nome dell'utente e `DELETED_CONTENT_PLACEHOLDER` per il commento. Questi possono essere personalizzati tramite l'interfaccia di personalizzazione del widget.

### Esempi

[inline-code-attrs-start title = 'Esempio cURL di rimozione SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta di rimozione SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // predefinito
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Puoi impostare questo su true per cancellare anche i commenti dell'utente. Questo raddoppierà il costo in crediti. **/
    deleteComments?: 'true' | 'false'
    /** Puoi impostarlo a piacere per determinare come gestire i commenti dell'utente. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta di rimozione SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Incluso in caso di errore. **/
    reason?: string
    user?: SSOUser; // Restituiamo l'utente rimosso in caso di successo.
}
[inline-code-end]

---