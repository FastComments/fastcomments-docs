[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Elenca i voti individuali dietro i conteggi di un sondaggio, dal più vecchio al più recente. Un credito per ogni 100 voti restituiti.

Un sondaggio appartiene a un commento, quindi i voti vengono letti un sondaggio alla volta e `commentId` è obbligatorio. Restringi ulteriormente con `voterId` per verificare come ha votato una persona, o con `optionId` per elencare tutti coloro che hanno scelto una determinata opzione.

Al massimo vengono restituiti 1000 voti per chiamata. Usa `skip` per scorrere le pagine successive.

L'impostazione `privacy` del sondaggio è rispettata: i voti su un sondaggio anonimo non possono essere letti e la richiesta fallisce con `poll-anonymous`. Consulta la struttura `PollVote` per i dettagli.

[inline-code-attrs-start title = 'Esempio cURL di PollVotes Get'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta PollVotes Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta PollVotes Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Incluso in caso di errore. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Conteggio dei Voti per Opzione

Non è necessario sommare questi valori per ottenere i risultati – il sondaggio contiene i propri conteggi. Leggi il sondaggio con `GET /api/v1/polls/:commentId` invece, e utilizza questa API quando hai bisogno di sapere chi ha votato.

### Ogni Sondaggio su una Pagina

Non esiste un elenco di voti a livello di pagina. Per fare un report su un'intera pagina, recupera i suoi commenti con `GET /api/v1/comments`, che restituisce il sondaggio di ogni commento e i relativi conteggi, quindi leggi i voti per i sondaggi di tuo interesse.