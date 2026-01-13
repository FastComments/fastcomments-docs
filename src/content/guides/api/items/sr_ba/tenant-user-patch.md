[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава ажурирање појединачног `TenantUser`.

Ажурирање `TenantUser` има сљедећа ограничења:

- `signUpDate` не смије бити у будућности.
- `locale` мора бити на листи [Podržane lokalne postavke](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` мора бити јединствен на цијелом FastComments.com. Ако је то проблем, предлажемо кориштење SSO уместо тога.
- `email` мора бити јединствен на цијелом FastComments.com. Ако је то проблем, предлажемо кориштење SSO уместо тога.
- Не можете ажурирати `tenantId` корисника.

Можемо креирати `TenantUser` на сљедећи начин

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za ažuriranje TenantUser-a'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za ažuriranje TenantUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Kada se промијени email или username, можете поставити ово на 'true' да такође ажурирате коментаре корисника. Ово ће удвостручити трошак кредита. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za ažурирање TenantUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Укључено у случају неуспјеха. **/
    reason?: string
}
[inline-code-end]

---