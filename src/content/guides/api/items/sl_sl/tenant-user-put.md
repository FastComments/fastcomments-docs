[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Ta pot omogoča zamenjavo enega `TenantUser`.

Zamenjava `TenantUser` ima naslednje omejitve:

- `signUpDate` ne sme biti v prihodnosti.
- `locale` mora biti na seznamu [Podprti lokali](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` mora biti edinstven v celotnem FastComments.com. Če je to težava, priporočamo uporabo SSO.
- `email` mora biti edinstven v celotnem FastComments.com. Če je to težava, priporočamo uporabo SSO.
- Ne morete posodobiti `tenantId` uporabnika.

`TenantUser` lahko ustvarimo na naslednji način

[inline-code-attrs-start title = 'Primer cURL zahteve za zamenjavo TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za zamenjavo TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Ko se spremenita email ali username, lahko to nastavite na 'true', da posodobite tudi komentarje uporabnika. To bo podvojilo stroške kreditov. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za zamenjavo TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Vključeno v primeru napake. **/
    reason?: string
}
[inline-code-end]

---