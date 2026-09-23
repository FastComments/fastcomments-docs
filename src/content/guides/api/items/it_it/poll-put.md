[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Allega un sondaggio a un commento esistente, o imposta lo stato completo del sondaggio che ha già.

Il corpo è il sondaggio completo, e le opzioni che invii diventano le opzioni del sondaggio, in quell'ordine. Ogni opzione è associata al suo `id`:

- Un'opzione inviata con l'`id` di un'opzione esistente mantiene quell'opzione e i suoi voti. La sua etichetta e posizione vengono aggiornate a quanto hai inviato.
- Un'opzione inviata senza `id` viene aggiunta, senza voti.
- Un'opzione esistente che ometti viene rimossa, insieme ai voti espressi su di essa. `totalVotes` diminuisce della stessa quantità.

Quindi, per aggiungere un'opzione, invia le opzioni attuali con i loro id più la nuova senza id. Per rimuoverne una, invia l'elenco senza di essa. Gli id delle opzioni sono presenti nel sondaggio restituito da `GET /api/v1/polls/:commentId`.

Inviare nessun id sostituisce ogni opzione e elimina tutti i voti già espressi sul sondaggio. Se il sondaggio ha voti, ciò richiede `replaceVotes=true`, e senza di esso l'API risponde con `replace-votes-required`.

Anche gli altri campi vengono sostituiti: omettere `closesAt`, `privacy` o `requireVoteToSeeResults` li ripristina al valore predefinito. Per modificare un singolo campo e lasciare gli altri invariati, usa `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Esempio cURL Poll Put'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Necessario per mantenere nessuno degli id delle opzioni esistenti quando il sondaggio ha voti, poiché ciò li elimina tutti. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** L'id di un'opzione esistente, per mantenerla e i suoi voti. Ometti per aggiungere una nuova opzione. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** L'elenco completo e ordinato. Le opzioni esistenti omesse vengono rimosse insieme ai loro voti. **/
    options: PollPutOption[]
    /** Deve essere nel futuro quando il commento non ha ancora un sondaggio. Ometti per un sondaggio che rimane aperto. **/
    closesAt?: string | null
    /** 0 anonimo (predefinito), 1 amministratori e moderatori, 2 tutti. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Incluso in caso di errore. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Other Notes

- Un `id` che non è presente nel sondaggio, o lo stesso `id` fornito due volte, fallisce con `poll-invalid`. Un commento senza sondaggio non ha ancora id delle opzioni, quindi ogni opzione inviata deve omettere `id`.
- La privacy del sondaggio può essere ridotta ma non ampliata una volta che ha voti.
- Questa API rispetta le impostazioni del tuo sito. Se i sondaggi non sono abilitati per il sito o la pagina, fallisce con `polls-disabled`.
- Un commento bloccato non può avere il suo sondaggio modificato, e fallisce con `locked`.
- I widget collegati vengono aggiornati in tempo reale, così gli spettatori vedono il nuovo sondaggio senza ricaricare.