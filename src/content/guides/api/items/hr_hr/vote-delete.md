[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje brisanje jednog `Vote`.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za brisanje glasa'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za brisanje glasa'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje glasa'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]

Notes:

- Ovaj API poštuje postavke na razini tenant-a. Na primjer, ako onemogućite glasovanje za određenu stranicu i pokušate stvoriti glas putem API-ja, to će rezultirati pogreškom s kodom `voting-disabled`.
- Ovaj API je prema zadanim postavkama aktivan.
- Ovaj API će ažurirati `votes` odgovarajućeg `Comment`.