[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Questo endpoint API fornisce la possibilità di segnalare (flaggare) un commento per un utente specifico.

Note:

- Questa chiamata deve sempre essere effettuata nel contesto di un utente. L'utente può essere un Utente FastComments.com, un Utente SSO o un Utente Tenant.
- Se è impostata una soglia di flag-per-nascondere, il commento verrà nascosto automaticamente in tempo reale dopo essere stato segnalato il numero definito di volte.
- Dopo che è stato automaticamente disapprovato (nascosto), il commento può essere riapprovato solo da un amministratore o un moderatore. La rimozione della segnalazione non riapproverà il commento.

[inline-code-attrs-start title = 'Esempio cURL di segnalazione commento'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Per la segnalazione anonima, dobbiamo specificare un `anonUserId`. Questo può essere un ID che rappresenta la sessione anonima, oppure un UUID casuale.
Questo ci permette di supportare la segnalazione e la rimozione della segnalazione dei commenti anche se un utente non ha effettuato l'accesso. In questo modo, il commento può essere
contrassegnato come segnalato quando i commenti vengono recuperati con lo stesso `anonUserId`.

[inline-code-attrs-start title = 'Esempio cURL di segnalazione commento anonimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struttura della richiesta di segnalazione commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta di segnalazione commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Incluso in caso di errore. **/
    reason?: string
    /** Il commento è stato disapprovato (nascosto) a causa di troppe segnalazioni? **/
    wasUnapproved?: boolean;
}
[inline-code-end]