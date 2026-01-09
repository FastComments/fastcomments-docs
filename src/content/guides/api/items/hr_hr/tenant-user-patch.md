[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje ažuriranje jednog `TenantUser`.

Ažuriranje `TenantUser` ima sljedeća ograničenja:

- `signUpDate` ne smije biti u budućnosti.
- `locale` mora biti na popisu [Podržane lokalizacije](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` mora biti jedinstven na FastComments.com. Ako je to problem, predlažemo korištenje SSO umjesto toga.
- `email` mora biti jedinstven na FastComments.com. Ako je to problem, predlažemo korištenje SSO umjesto toga.
- Ne možete ažurirati `tenantId` korisnika.

Možemo stvoriti `TenantUser` na sljedeći način

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za ažuriranje TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za ažuriranje TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Kada se promijeni email ili korisničko ime, možete ovo postaviti na true da biste također ažurirali korisnikove komentare. To će udvostručiti trošak kredita. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za ažuriranje TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]

---