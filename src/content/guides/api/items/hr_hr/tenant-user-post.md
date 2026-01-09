[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje dodavanje jednog `TenantUser`.

Stvaranje `TenantUser` ima sljedeća ograničenja:

- `username` je obavezan.
- `email` je obavezan.
- `signUpDate` ne smije biti u budućnosti.
- `locale` mora biti na popisu [Podržane lokalizacije](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` mora biti jedinstven na cijelom FastComments.com. Ako je to problem, preporučujemo korištenje SSO.
- `email` mora biti jedinstven na cijelom FastComments.com. Ako je to problem, preporučujemo korištenje SSO.
- Ne smijete stvoriti više tenant korisnika nego što je definirano pod `maxTenantUsers` u vašem paketu. 

Možemo stvoriti `TenantUser` na sljedeći način

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za stvaranje TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za stvaranje TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za stvaranje TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Uključeno pri neuspjehu. **/
    reason?: string
    tenantUser?: TenantUser; // Vraćamo kompletno kreiranog tenant korisnika pri uspjehu.
}
[inline-code-end]

---