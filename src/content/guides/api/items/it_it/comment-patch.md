[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Questo endpoint API fornisce la possibilità di aggiornare un singolo commento.

Note:

- Questa API può aggiornare il widget dei commenti "live" se desiderato (questo aumenta il `creditsCost` base da `1` a `2`).
  - Questo può rendere "live" la migrazione dei commenti tra pagine (modificando `urlId`).
  - Le migrazioni costano `2` crediti aggiuntivi poiché le pagine vengono precalcolate e questo è intensivo per la CPU.
- A differenza dell'API di creazione, questa API NON creerà automaticamente oggetti utente nel nostro sistema se viene fornita un'email.
- I commenti aggiornati tramite questa API possono comunque essere controllati per spam, se desiderato.
- Configurazioni come la lunghezza massima del commento, se configurate tramite la pagina admin Customization Rule, si applicheranno qui.
- Per consentire agli utenti di aggiornare il testo del loro commento, puoi semplicemente specificare `comment` nel corpo della richiesta. Genereremo il relativo `commentHTML`.
  - Se definisci sia `comment` che `commentHTML` non genereremo automaticamente l'HTML.
  - Se l'utente aggiunge menzioni o hashtag nel nuovo testo, verranno comunque elaborati come nell'API `POST`.
- Quando si aggiorna `commenterEmail` su un commento, è consigliabile specificare anche `userId`. Altrimenti, devi assicurarti che l'utente con questa email appartenga al tuo tenant, altrimenti la richiesta fallirà.  
- Se il commento target è bloccato (`isLocked: true`), la richiesta viene rifiutata con `code: 'locked'`. Sblocca prima il commento, aggiornalo, poi richiudilo se desiderato.


[inline-code-attrs-start title = 'Esempio minimo di richiesta cURL PATCH per Commento'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta PATCH per Commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** L'utente che esegue l'aggiornamento. Se desiderato può essere usato per verificare che possa modificare il commento.  **/
    contextUserId?: string
	/** Dobbiamo controllare se il nuovo commento sembra spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Se il commento dovrebbe apparire "live" agli utenti che visualizzano istanze del widget dei commenti con lo stesso urlId. NOTA: Raddoppia il costo in crediti da 1 a 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta PATCH per Commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Incluso in caso di errore. **/
    reason?: string
}
[inline-code-end]