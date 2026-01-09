[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Dit API-eindpunt maakt het mogelijk om de vlag van een reactie voor een specifieke gebruiker te verwijderen.

Opmerkingen:

- Deze oproep moet altijd worden gedaan in de context van een gebruiker. De gebruiker kan een FastComments.com User, SSO User, or Tenant User zijn.
- Nadat een reactie automatisch is afgekeurd (verborgen) - kan de reactie alleen opnieuw worden goedgekeurd door een beheerder of moderator. Het verwijderen van de vlag zal de reactie niet opnieuw goedkeuren.

[inline-code-attrs-start title = 'Voorbeeld cURL-aanvraag: vlag van reactie verwijderen'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Voor anoniem vlaggen moeten we een `anonUserId` opgeven. Dit kan een ID zijn die de anonieme sessie vertegenwoordigt, of een willekeurige UUID.

[inline-code-attrs-start title = 'Voorbeeld cURL-aanvraag: anonieme reactie vlag verwijderen'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Structuur van het verzoek: vlag van reactie verwijderen'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van de respons: vlag van reactie verwijderen'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]