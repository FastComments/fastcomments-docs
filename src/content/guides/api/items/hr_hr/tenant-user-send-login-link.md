[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Ovaj endpoint omogućuje slanje login poveznice (login link) pojedinom `TenantUser`-u.

Koristan kada masovno kreirate korisnike i ne želite objašnjavati kako da se prijave na FastComments.com. Ovo će im poslati "magic link" za prijavu koji
istječe nakon `30 days`.

Sljedeća ograničenja vrijede za slanje login poveznice `TenantUser`-u:
- `TenantUser` mora već postojati.
- Morate imati pristup za upravljanje `Tenant`-om kojem `TenantUser` pripada.

Login poveznicu `TenantUser`-u možemo poslati na sljedeći način:

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za TenantUser login link'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Ovo će poslati e-poštu poput `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struktura zahtjeva za TenantUser login link'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za TenantUser login link'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]