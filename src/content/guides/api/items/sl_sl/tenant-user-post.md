[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Ta končna točka omogoča dodajanje enega `TenantUser`.

Ustvarjanje `TenantUser` ima naslednje omejitve:

- `username` je zahtevan.
- `email` je zahtevan.
- `signUpDate` ne sme biti v prihodnosti.
- `locale` mora biti na seznamu [Podprte lokalne nastavitve](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` mora biti edinstven na celotnem FastComments.com. Če to predstavlja težavo, priporočamo uporabo SSO.
- `email` mora biti edinstven na celotnem FastComments.com. Če to predstavlja težavo, priporočamo uporabo SSO.
- Ne smete ustvariti več uporabnikov najemnika, kot je določeno v `maxTenantUsers` v vašem paketu. 

TenantUser lahko ustvarite na naslednji način

[inline-code-attrs-start title = 'Primer cURL zahteve za ustvarjanje TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtevka za ustvarjanje TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za ustvarjanje TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Vključeno ob neuspehu. **/
    reason?: string
    tenantUser?: TenantUser; // Ob uspehu vrnemo popolnoma ustvarjenega uporabnika najemnika.
}
[inline-code-end]