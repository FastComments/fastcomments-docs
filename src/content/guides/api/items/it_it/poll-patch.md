[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Modifica un sondaggio senza alterare i suoi voti. Usa questa operazione per correggere un errore di battitura nella domanda o in un'opzione, per chiudere o riaprire il sondaggio, o per cambiare chi può vedere chi ha votato.

Le opzioni sono identificate dal loro `id`, e un `PATCH` rinomina quelle che specifichi. Per aggiungere, rimuovere o riordinare le opzioni, invia l'elenco completo delle opzioni a `PUT /api/v1/polls/:commentId`: le opzioni che invii con i loro id mantengono anche i loro voti.

Ogni campo è opzionale, ma ne deve essere fornito almeno uno.

[inline-code-attrs-start title = 'Esempio cURL Patch Sondaggio'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Esempio cURL Chiudi Sondaggio Ora'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta Patch Sondaggio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Relabels existing options. Every id given must already be on the poll. **/
    options?: { id: string, label: string }[] | null
    /** A date in the past closes the poll now. null reopens a closed poll. **/
    closesAt?: string | null
    /** 0 anonymous, 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta Patch Sondaggio'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Altre Note

- Assegnare un id di opzione che non è presente nel sondaggio fallisce con `poll-invalid` invece di non fare nulla silenziosamente.
- Le etichette devono rimanere uniche all'interno del sondaggio, contando anche le opzioni che non stai modificando.
- A differenza della creazione di un sondaggio, `closesAt` può essere nel passato qui - è così che si chiude un sondaggio immediatamente.
- La privacy del sondaggio può essere ridotta ma non ampliata una volta che ha dei voti.
- Un commento bloccato non può avere il suo sondaggio modificato, e fallisce con `locked`.