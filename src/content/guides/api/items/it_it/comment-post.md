[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Questo endpoint API consente di creare commenti.

Gli usi comuni sono interfacce utente personalizzate, integrazioni o importazioni.

Note:

- Questa API può aggiornare il widget dei commenti in "live" se desiderato (questo aumenta `creditsCost` da `1` a `2`).
- Questa API creerà automaticamente oggetti utente nel nostro sistema se viene fornita un'email.
- Tentare di salvare due commenti con email diverse, ma lo stesso username, risulterà in un errore per il secondo commento.
- Se specifichi `parentId`, e un commento figlio ha `notificationSentForParent` impostato su false, **invieremo notifiche per il commento genitore**. Questo viene fatto ogni ora (raggruppiamo le notifiche insieme per diminuire il numero di email inviate).
- Se vuoi inviare email di benvenuto quando crei utenti, o email di verifica del commento, imposta `sendEmails` su `true` nei parametri di query.
- I commenti creati tramite questa API appariranno nelle pagine Analytics e Moderation dell'app di amministrazione.
- Le "bad words" sono ancora mascherate nei nomi dei commentatori e nel testo del commento se l'impostazione è attivata.
- I commenti creati tramite questa API possono comunque essere controllati per spam, se desiderato.
- La configurazione come la lunghezza massima del commento, se configurata tramite la pagina admin Customization Rule, si applicherà qui.

I dati minimi richiesti per l'invio che verranno visualizzati nel widget dei commenti, sono i seguenti:

[inline-code-attrs-start title = 'Esempio minimo di richiesta cURL per POST commento'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

Una richiesta più realistica potrebbe apparire così:

[inline-code-attrs-start title = 'Esempio di richiesta cURL per POST commento'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta POST del commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Se il commento deve apparire "live" agli utenti che visualizzano istanze del widget dei commenti con lo stesso urlId. NOTA: raddoppia il costo in crediti da 1 a 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta POST del commento'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Incluso in caso di errore. **/
    reason?: string
    /** Il commento creato. **/
    comment?: Comment
    /** L'utente associato, che potrebbe o meno essere già esistito. **/
    user?: User
}
[inline-code-end]

---