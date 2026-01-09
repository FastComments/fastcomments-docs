[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Questo endpoint API fornisce la possibilità di rimuovere la segnalazione di un commento per uno specifico utente.

Note:

- Questa chiamata deve sempre essere effettuata nel contesto di un utente. L'utente può essere un FastComments.com User, SSO User, o Tenant User.
- Dopo che un commento è stato automaticamente non approvato (nascosto) - il commento può essere ri-approvato solo da un amministratore o moderatore. La rimozione della segnalazione (un-flag) non ri-approverà il commento.

[inline-code-attrs-start title = 'Esempio cURL per la rimozione della segnalazione di un commento'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Per la segnalazione anonima, dobbiamo specificare un `anonUserId`. Questo può essere un ID che rappresenta la sessione anonima, o un UUID casuale.

[inline-code-attrs-start title = 'Esempio cURL per la rimozione della segnalazione di un commento anonimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struttura della richiesta per la rimozione della segnalazione di un commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta per la rimozione della segnalazione di un commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di fallimento. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Incluso in caso di fallimento. **/
    reason?: string
}
[inline-code-end]