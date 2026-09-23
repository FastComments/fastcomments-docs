[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Registra un voto su un sondaggio.

Un elettore può avere al massimo un voto per sondaggio. Richiamare nuovamente questo endpoint per lo stesso elettore sposta il suo voto alla nuova opzione invece di aggiungerne un secondo, e votare per l'opzione che ha già scelto non ha alcun effetto.

La risposta include il sondaggio, così ottieni i conteggi aggiornati senza una seconda richiesta.

[inline-code-attrs-start title = 'Esempio cURL Creazione PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Esempio cURL Creazione PollVote Anonimo'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura Richiesta Creazione PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** One of userId or anonUserId is required. **/
    userId?: string
    anonUserId?: string
    /** The end user's IP, used for the anonymous rate limit. Defaults to the caller's IP. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura Risposta Creazione PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Included on failure. **/
    reason?: string
    pollVote: PollVote
    /** The poll with its updated counts. **/
    poll: CommentPoll
}
[inline-code-end]

### Voti Anonimi

Imposta `anonUserId` invece di `userId` per registrare un voto per qualcuno che non è connesso. quell'ID non deve corrispondere a un utente da nessuna parte – identifica semplicemente la sessione, così la stessa persona non viene contata due volte.

Il voto anonimo deve essere abilitato per il tuo sito. Se il voto è limitato agli utenti connessi, un voto con solo un `anonUserId` fallisce con `poll-login-required`.

I voti anonimi sono anche limitati per frequenza per IP per sondaggio, per impedire a una persona di riempire un sondaggio cancellando la propria sessione. Invia l'`ip` dell'utente finale in modo che il limite si applichi a lui anziché al tuo server.

### Altre Note

- Un `userId` deve corrispondere a un utente che esiste sul tuo sito. I voti per un utente appartenente a un altro sito vengono rifiutati.
- Votare su un sondaggio chiuso fallisce con `poll-closed`.
- Questa API aggiorna i conteggi del sondaggio e li invia in tempo reale ai widget collegati.

---