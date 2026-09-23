[api-resource-header-start name = 'Poll'; route = 'DELETE /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Verwijdert een poll van zijn commentaar, samen met elke stem die erop is uitgebracht. Het commentaar zelf blijft onaangetast.

Het verwijderen van het commentaar verwijdert ook de poll en stemmen, dus dit is alleen nodig wanneer je het commentaar wilt behouden.

[inline-code-attrs-start title = 'Voorbeeld cURL-verzoek voor poll verwijderen'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van poll‑verwijderingsverzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van poll‑verwijderingsrespons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij een fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Opgenomen bij een fout. **/
    reason?: string
}
[inline-code-end]