[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Listet die einzelnen Stimmen hinter den Zählungen einer Umfrage auf, älteste zuerst. Ein Credit pro 100 zurückgegebenen Stimmen.

Eine Umfrage gehört zu einem Kommentar, daher werden die Stimmen immer eine Umfrage nach der anderen gelesen und `commentId` ist erforderlich. Grenzen Sie die Ergebnisse weiter ein
mit `voterId`, um zu prüfen, wie eine bestimmte Person abgestimmt hat, oder mit `optionId`, um alle aufzulisten, die eine bestimmte Option gewählt haben.

Es werden höchstens 1000 Stimmen pro Aufruf zurückgegeben. Verwenden Sie `skip`, um durch weitere Ergebnisse zu blättern.

Die `privacy`-Einstellung der Umfrage wird respektiert: Die Stimmen einer anonymen Umfrage können nicht gelesen werden, und die Anfrage schlägt
mit `poll-anonymous` fehl. Details finden Sie in der `PollVote`-Struktur.

[inline-code-attrs-start title = 'PollVotes Get cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVotes Get Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Included on failure. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Stimmen pro Option zählen

Sie müssen diese nicht zusammenrechnen, um die Ergebnisse zu erhalten - die Umfrage führt ihre eigenen Zählungen. Lesen Sie die Umfrage stattdessen mit
`GET /api/v1/polls/:commentId` und verwenden Sie diese API, wenn Sie wissen möchten, wer abgestimmt hat.

### Jede Umfrage auf einer Seite

Es gibt keine seitenweite Auflistung der Stimmen. Um einen Bericht über eine ganze Seite zu erstellen, rufen Sie deren Kommentare mit
`GET /api/v1/comments` ab, was die Umfrage jedes Kommentars und dessen Zählungen zurückgibt, und lesen Sie dann die Stimmen für die
Umfragen, die Sie interessieren.