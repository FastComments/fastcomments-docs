---
[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućava brisanje pojedinačnog `Vote`.

[inline-code-attrs-start title = 'Primer cURL zahteva za brisanje Vote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za brisanje Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Uključeno u slučaju greške. **/
    reason?: string
}
[inline-code-end]

Napomene:

- Ovaj API poštuje postavke na nivou tenanta. Na primer, ako onemogućite glasanje za određenu stranicu, i pokušate da kreirate glas preko API-ja, to će završiti greškom sa kodom `voting-disabled`.
- Ovaj API je podrazumevano aktivan.
- Ovaj API će ažurirati `votes` odgovarajućeg `Comment`.

---