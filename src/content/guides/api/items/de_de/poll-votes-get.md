[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Listet die einzelnen Stimmen hinter den Zählerwerten einer Umfrage auf, älteste zuerst. Ein Guthaben pro 100 zurückgegebene Stimmen.

Eine Umfrage gehört zu einem Kommentar, daher werden Stimmen jeweils einer Umfrage nach gelesen und `commentId` ist erforderlich. Eingrenzen kann man weiter mit `voterId`, um zu prüfen, wie eine Person abgestimmt hat, oder mit `optionId`, um alle aufzulisten, die eine bestimmte Option gewählt haben.

Pro Aufruf werden maximal 1000 Stimmen zurückgegeben. Verwenden Sie `skip`, um weitere Seiten abzurufen.

Die `privacy`‑Einstellung der Umfrage wird respektiert: Die Stimmen einer anonymen Umfrage können nicht gelesen werden, und die Anfrage schlägt mit `poll-anonymous` fehl. Siehe die `PollVote`‑Struktur für Details.

[inline-code-attrs-start title = 'PollVotes Get cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Counting Votes Per Option

Sie müssen diese nicht zusammenzählen, um die Ergebnisse zu erhalten – die Umfrage enthält ihre eigenen Zähler. Lesen Sie stattdessen die Umfrage mit `GET /api/v1/polls/:commentId` und verwenden Sie diese API, wenn Sie wissen müssen, wer abgestimmt hat.

### Every Poll On A Page

Es gibt keine seitenweite Auflistung der Stimmen. Um über eine gesamte Seite zu berichten, rufen Sie deren Kommentare mit `GET /api/v1/comments` ab, die für jeden Kommentar die Umfrage und deren Zähler zurückgeben, und lesen Sie anschließend die Stimmen der Umfragen, die Sie interessieren.