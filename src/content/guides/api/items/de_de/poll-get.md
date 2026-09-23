[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Liest die an einen Kommentar angehängte Umfrage, mit ihren aktuellen Stimmenzahlen.

Umfragen werden auch im Kommentar selbst von den Kommentar-APIs zurückgegeben, verwenden Sie dies also, wenn Sie nur die Ergebnisse
und nicht den gesamten Kommentar benötigen.

[inline-code-attrs-start title = 'Poll Get cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Poll Get Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll Get Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

Ein Kommentar, der keine Umfrage hat, ein gelöschter Kommentar und eine Kommentar-ID, die nicht existiert, antworten alle
auf dieselbe Weise, mit `poll-not-found`.

---