[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava dodavanje jednog `TenantUser`.

Kreiranje `TenantUser`-a ima sljedeća ograničenja:

- `username` je obavezno.
- `email` je obavezno.
- `signUpDate` ne smije biti u budućnosti.
- `locale` mora biti na listi [Supported Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` mora biti jedinstven na cijelom FastComments.com. Ako je to problem, preporučujemo korištenje SSO umjesto toga.
- `email` mora biti jedinstven na cijelom FastComments.com. Ako je to problem, preporučujemo korištenje SSO umjesto toga.
- Ne smijete kreirati više tenant korisnika nego što je definirano u `maxTenantUsers` u vašem paketu. 

Možemo kreirati `TenantUser` na sljedeći način

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za kreiranje TenantUser-a'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za kreiranje TenantUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora pri kreiranju TenantUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    tenantUser?: TenantUser; // U slučaju uspjeha vraćamo kompletno kreiranog TenantUser-a.
}
[inline-code-end]