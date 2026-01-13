[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Questo endpoint API fornisce la possibilità di sbloccare un utente che ha scritto un determinato commento. Supporta lo sblocco di commenti scritti da FastComments.com Users, SSO Users, e Tenant Users.

Supporta un parametro body `commentIdsToCheck` per verificare se altri commenti potenzialmente visibili sul client dovrebbero essere bloccati/sbloccati dopo che questa azione è stata eseguita.

Note:

- Questa chiamata deve essere sempre effettuata nel contesto di un utente. L'utente può essere un FastComments.com User, SSO User, o Tenant User.
- Il `userId` nella richiesta è l'utente che sta *effettuando lo sblocco*. Per esempio: `User A` vuole sbloccare `User B`. Passa `userId=User A` e l'id del commento che `User B` ha scritto.
- I commenti completamente anonimi (nessun user id, nessuna email) non possono essere bloccati e verrà restituito un errore.

[inline-code-attrs-start title = 'Esempio cURL Sblocco Commento'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Esempio cURL Sblocco Commento Anonimo'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura Richiesta Sblocco Commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura Risposta Sblocco Commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Incluso in caso di errore. **/
    reason?: string
    /** Se commentIdsToCheck è definito, le voci in questa mappa con valore true sono ancora bloccate. Se false, potresti voler rendere nuovamente visibili i commenti all'utente in modo che non debba aggiornare. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]