[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Returnerer antallet for hver reaktion på en side, og hvilke reaktioner den aktuelle bruger har tilføjet.

[inline-code-attrs-start title = 'Page Reacts cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI-kodet JSON af dit SSO-objekt. Udelad for anonyme brugere. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts Responsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Inkluderet ved fejl. **/
    code?: string
    /** Inkluderet ved fejl. **/
    reason?: string
    /** Tælling pr. reaktions-id, for eksempel {"heart": 12, "laugh": 3}. Ikke angivet når siden ikke har nogen reaktioner. **/
    counts?: Record<string, number>
    /** Reaktions-id'erne som brugeren der laver anmodningen har tilføjet. Ikke angivet når de ikke har tilføjet nogen. **/
    reactedIds?: string[]
}
[inline-code-end]