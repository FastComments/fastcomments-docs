[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava zamenu jednog `TenantUser`.

Zamena `TenantUser` ima sledeća ograničenja:

- `signUpDate` ne sme biti u budućnosti.
- `locale` mora biti na listi [Supported Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` mora biti jedinstveno na celom FastComments.com. Ako je to problem, predlažemo korišćenje SSO-a umesto toga.
- `email` mora biti jedinstven na celom FastComments.com. Ako je to problem, predlažemo korišćenje SSO-a umesto toga.
- Ne možete promeniti `tenantId` korisnika.

Možemo kreirati `TenantUser` na sledeći način

[inline-code-attrs-start title = 'Primer cURL zahteva za zamenu TenantUser-a'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za zamenu TenantUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Kada se promeni email ili username, možete ovo postaviti na 'true' da biste takođe ažurirali komentare korisnika. Ovo će udvostručiti trošak kredita. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora pri zameni TenantUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Uključeno pri neuspehu. **/
    reason?: string
}
[inline-code-end]