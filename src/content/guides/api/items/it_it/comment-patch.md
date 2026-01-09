[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Questo endpoint API consente di aggiornare un singolo commento.

Note:

- Questa API può aggiornare il widget dei commenti "live" se desiderato (questo aumenta il `creditsCost` base da `1` a `2`).
  - Questo può rendere "live" la migrazione dei commenti tra pagine (cambiando `urlId`).
  - Le migrazioni costano ulteriori `2` crediti poiché le pagine vengono precalcolate ed è intensivo in termini di CPU.
- A differenza dell'API di creazione, questa API NON creerà automaticamente oggetti utente nel nostro sistema se viene fornita un'email.
- I commenti aggiornati tramite questa API possono ancora essere controllati per spam se desiderato.
- Le configurazioni come la lunghezza massima del commento, se impostate tramite la pagina di amministrazione Customization Rule, si applicheranno qui.
- Per consentire agli utenti di aggiornare il testo del loro commento, è sufficiente specificare `comment` nel corpo della richiesta. Genereremo il `commentHTML` risultante.
  - Se definisci sia `comment` sia `commentHTML`, non genereremo automaticamente l'HTML.
  - Se l'utente aggiunge menzioni o hashtag nel nuovo testo, verrà comunque elaborato come nell'API `POST`.
- Quando si aggiorna `commenterEmail` su un commento, è meglio specificare anche `userId`. Altrimenti, devi assicurarti che l'utente con questa email appartenga al tuo tenant, altrimenti la richiesta fallirà.  


[inline-code-attrs-start title = 'Esempio cURL minimo per PATCH del commento'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta PATCH del commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** L'utente che effettua l'aggiornamento. Se desiderato può essere usato per verificare che possa modificare il commento.  **/
    contextUserId?: string
	/** Dovremmo verificare se il nuovo commento sembra spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta PATCH del commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di fallimento. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Incluso in caso di fallimento. **/
    reason?: string
}
[inline-code-end]