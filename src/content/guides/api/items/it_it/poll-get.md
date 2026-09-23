[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Legge il sondaggio allegato a un commento, con i conteggi dei voti attuali.

I sondaggi sono restituiti anche sul commento stesso dalle API dei commenti, quindi usa questo quando vuoi solo i risultati
e non l'intero commento.

[inline-code-attrs-start title = 'Esempio cURL per Ottenere il Sondaggio'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Richiesta Poll Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della Risposta Poll Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Incluso in caso di errore. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

Un commento che non ha sondaggio, un commento che è stato eliminato e un ID commento che non esiste rispondono
allo stesso modo, con `poll-not-found`.

---