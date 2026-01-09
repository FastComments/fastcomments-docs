[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Ta ruta omogoča brisanje posameznega `Vote`.

[inline-code-attrs-start title = 'Primer cURL zahteve za brisanje glasu'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za brisanje glasu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora pri brisanju glasu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Vključeno v primeru napake. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Vključeno v primeru napake. **/
    reason?: string
}
[inline-code-end]

Opombe:

- Ta API upošteva nastavitve na ravni najemnika. Na primer, če onemogočite glasovanje za določeno stran in poskušate ustvariti glas prek API-ja, bo to neuspešno in se bo vrnila koda napake `voting-disabled`.
- Ta API je privzeto aktiven.
- Ta API bo posodobil `votes` ustreznega `Comment`.