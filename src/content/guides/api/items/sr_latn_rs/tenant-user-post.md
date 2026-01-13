[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućava dodavanje jednog `TenantUser`.

Kreiranje `TenantUser` ima sledeća ograničenja:

- `username` je obavezan.
- `email` je obavezan.
- `signUpDate` ne sme biti u budućnosti.
- `locale` mora biti na listi [Podržane lokalizacije](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` mora biti jedinstven na celoj FastComments.com. Ako je ovo problem, predlažemo korišćenje SSO umesto toga.
- `email` mora biti jedinstven na celoj FastComments.com. Ako je ovo problem, predlažemo korišćenje SSO umesto toga.
- Ne smete kreirati više tenant korisnika nego što je definisano pod `maxTenantUsers` u vašem paketu. 

Možemo kreirati `TenantUser` na sledeći način

[inline-code-attrs-start title = 'TenantUser - primer cURL zahteva za kreiranje'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za kreiranje TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za kreiranje TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Uključeno u slučaju greške. **/
    reason?: string
    tenantUser?: TenantUser; // Vraćamo kompletno kreiranog tenant korisnika pri uspehu.
}
[inline-code-end]

---