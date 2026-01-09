[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Questo endpoint API consente di bloccare l'utente che ha scritto un determinato commento. Supporta il blocco per commenti scritti da FastComments.com Users, SSO Users, and Tenant Users.

Supporta un parametro nel body `commentIdsToCheck` per verificare se altri commenti potenzialmente visibili nel client debbano essere bloccati/sbloccati dopo l'esecuzione di questa azione.

Notes:

- Questa chiamata deve sempre essere effettuata nel contesto di un utente. L'utente può essere un FastComments.com User, SSO User, o Tenant User.
- Il `userId` nella richiesta è l'utente che sta *eseguendo il blocco*. Per esempio: `User A` vuole bloccare `User B`. Passa `userId=User A` e l'id del commento che `User B` ha scritto.
- Commenti completamente anonimi (no user id, no email) non possono essere bloccati e verrà restituito un errore.

[inline-code-attrs-start title = 'Esempio cURL di blocco commento'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Per il blocco anonimo, dobbiamo specificare un `anonUserId`. Questo può essere un ID che rappresenta la sessione anonima, o un UUID casuale.
Questo ci permette di supportare il blocco dei commenti anche se un utente non è autenticato recuperando i commenti con lo stesso `anonUserId`.

[inline-code-attrs-start title = 'Esempio cURL di blocco commento anonimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura richiesta blocco commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura risposta blocco commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di fallimento. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Incluso in caso di fallimento. **/
    reason?: string
    /** Se commentIdsToCheck è definito, le voci in questa mappa con valore true sono anch'esse bloccate. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---