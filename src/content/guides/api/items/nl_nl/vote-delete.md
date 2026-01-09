[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Deze route biedt de mogelijkheid om een enkele `Vote` te verwijderen.

[inline-code-attrs-start title = 'Voorbeeld cURL-verzoek voor Vote verwijderen'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van het Vote-verwijderingsverzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Vote Delete-respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]

Opmerkingen:

- Deze API respecteert instellingen op tenant-niveau. Bijvoorbeeld, als je stemmen uitschakelt voor een bepaalde pagina en je via de API probeert een stem te maken, zal dit mislukken met foutcode `voting-disabled`.
- Deze API is standaard actief.
- Deze API zal de `votes` van de overeenkomstige `Comment` bijwerken.