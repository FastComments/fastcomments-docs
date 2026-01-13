[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Ova ruta omogućava slanje linka za prijavu pojedinačnom `TenantUser`.

Koristan kada se korisnici kreiraju u batch režimu i ne želite da im objašnjavate kako da se prijave na FastComments.com. Ovo će im jednostavno poslati "magic link" za prijavu koji ističe nakon `30 days`.

Postoje sljedeća ograničenja za slanje linka za prijavu `TenantUser`-u:
- `TenantUser` mora već postojati.
- Morate imati pristup upravljanju `Tenant`-om kojem `TenantUser` pripada.

Možemo poslati link za prijavu `TenantUser`-u na sljedeći način:

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za link za prijavu TenantUser-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Ovo će poslati e-poruku poput `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struktura zahtjeva za link za prijavu TenantUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za link za prijavu TenantUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]