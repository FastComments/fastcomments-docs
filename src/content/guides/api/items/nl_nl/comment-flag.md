[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Deze API-endpoint biedt de mogelijkheid om een opmerking voor een specifieke gebruiker te markeren.

Opmerkingen:

- Deze oproep moet altijd worden gedaan in de context van een gebruiker. De gebruiker kan een FastComments.com-gebruiker, SSO-gebruiker of Tenant-gebruiker zijn.
- Als er een drempel is ingesteld waarbij een opmerking bij voldoende flags wordt verborgen, wordt de opmerking meteen automatisch verborgen nadat het gedefinieerde aantal flags is bereikt.
- Nadat de opmerking automatisch is afgekeurd (verborgen), kan deze alleen opnieuw worden goedgekeurd door een beheerder of moderator. Het verwijderen van de vlag zal de opmerking niet opnieuw goedkeuren.

[inline-code-attrs-start title = 'Voorbeeld cURL-aanvraag voor het markeren van een opmerking'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Voor anoniem flaggen moeten we een `anonUserId` opgeven. Dit kan een ID zijn die de anonieme sessie vertegenwoordigt, of een willekeurige UUID.
Dit stelt ons in staat om het markeren en demarkeren van opmerkingen te ondersteunen, zelfs als een gebruiker niet is ingelogd. Op deze manier kan de opmerking als
gemarkeerd worden weergegeven wanneer opmerkingen worden opgehaald met dezelfde `anonUserId`.

[inline-code-attrs-start title = 'Voorbeeld cURL-aanvraag voor het anoniem markeren van een opmerking'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Structuur van het verzoek voor commentaarmarkering'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van de response voor commentaarmarkering'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Opgenomen bij fout. **/
    reason?: string
    /** Werd de opmerking afgekeurd (verborgen) omdat deze te vaak is gemarkeerd? **/
    wasUnapproved?: boolean;
}
[inline-code-end]