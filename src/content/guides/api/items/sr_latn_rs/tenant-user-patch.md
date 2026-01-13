[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava ažuriranje pojedinačnog `TenantUser`.

Ažuriranje `TenantUser` ima sledeća ograničenja:

- `signUpDate` ne sme biti u budućnosti.
- `locale` mora biti na listi [Podržane lokalizacije](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` mora biti jedinstven na čitavom FastComments.com. Ako je to problem, predlažemo da umesto toga koristite SSO.
- `email` mora biti jedinstven na čitavom FastComments.com. Ako je to problem, predlažemo da umesto toga koristite SSO.
- Ne možete ažurirati `tenantId` korisnika.

Možemo ažurirati `TenantUser` na sledeći način

[inline-code-attrs-start title = 'Primer cURL zahteva za ažuriranje TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za ažuriranje TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Kada se email ili username promene, možete ovo postaviti na true da biste takođe ažurirali komentare korisnika. Ovo će udvostručiti trošak u kreditima. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za ažuriranje TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
}
[inline-code-end]

---