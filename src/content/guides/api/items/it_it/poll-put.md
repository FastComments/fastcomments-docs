[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Collega un sondaggio a un commento esistente, oppure imposta lo stato completo del sondaggio che ha già.

Il body è il sondaggio completo, e le opzioni che invii diventano le opzioni del sondaggio, in quell'ordine. Ogni opzione
viene confrontata tramite il suo `id`:

- Un'opzione inviata con l'`id` di un'opzione esistente mantiene quell'opzione e i suoi voti. La sua etichetta e la sua posizione
  vengono aggiornate con quanto inviato.
- Un'opzione inviata senza `id` viene aggiunta, senza voti.
- Un'opzione esistente che ometti viene rimossa, insieme ai voti espressi su di essa. `totalVotes` diminuisce della stessa
  quantità.

Quindi, per aggiungere un'opzione, invia le opzioni correnti con i loro id più quella nuova senza id. Per rimuoverne una,
invia l'elenco senza di essa. Gli id delle opzioni si trovano nel sondaggio restituito da `GET /api/v1/polls/:commentId`.

Non inviare alcun id sostituisce ogni opzione ed elimina tutti i voti già espressi sul sondaggio. Se il sondaggio ha
voti, questo richiede `replaceVotes=true`, e senza di essa l'API risponde con `replace-votes-required`.

Anche gli altri campi vengono sostituiti: omettere `closesAt`, `privacy` o `requireVoteToSeeResults` li ripristina al
loro valore predefinito. Per modificare un singolo campo lasciando invariato il resto, usa `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Esempio cURL di Poll Put'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struttura della richiesta Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Required to keep none of the existing option ids when the poll has votes, since that deletes them all. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** The id of an existing option, to keep it and its votes. Omit to add a new option. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** The complete, ordered list. Existing options left out are removed with their votes. **/
    options: PollPutOption[]
    /** Must be in the future when the comment has no poll yet. Omit for a poll that stays open. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Altre note

- Un `id` che non è presente nel sondaggio, o lo stesso `id` fornito due volte, causa un errore con `poll-invalid`. Un commento senza
  sondaggio non ha ancora id di opzioni, quindi ogni opzione inviata ad esso deve omettere `id`.
- La privacy del sondaggio può essere ristretta ma non ampliata una volta che ha voti.
- Questa API rispetta le impostazioni del tuo sito. Se i sondaggi non sono abilitati per il sito o la pagina, fallisce con
  `polls-disabled`.
- Un commento bloccato non può avere il suo sondaggio modificato, e fallisce con `locked`.
- I widget collegati vengono aggiornati in tempo reale, quindi gli spettatori vedono il nuovo sondaggio senza ricaricare.