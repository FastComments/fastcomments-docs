[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje zamjenu pojedinačnog `TenantUser`-a.

Zamjena `TenantUser`-a ima sljedeća ograničenja:

- Polje `signUpDate` ne smije biti u budućnosti.
- `locale` mora biti na popisu [Podržani lokaliteti](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` mora biti jedinstven na cijelom FastComments.com. Ako je to problem, preporučamo korištenje SSO-a.
- `email` mora biti jedinstven na cijelom FastComments.com. Ako je to problem, preporučamo korištenje SSO-a.
- Ne možete ažurirati `tenantId` korisnika.

Možemo zamijeniti `TenantUser` na sljedeći način

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za zamjenu TenantUser-a'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za zamjenu TenantUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Kada se promijeni email ili korisničko ime, možete postaviti ovo na true da biste također ažurirali korisnikove komentare. Ovo će udvostručiti trošak kredita. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za zamjenu TenantUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]