[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Ta pot omogoča posodobitev posameznega `TenantUser`.

Posodabljanje `TenantUser` ima naslednje omejitve:

- `signUpDate` ne sme biti v prihodnosti.
- `locale` mora biti na seznamu [Podprte lokalizacije](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- `username` mora biti edinstven po celotnem FastComments.com. Če je to težava, priporočamo uporabo SSO.
- `email` mora biti edinstven po celotnem FastComments.com. Če je to težava, priporočamo uporabo SSO.
- Ne morete posodobiti `tenantId` uporabnika.

Ustvarimo lahko `TenantUser` na naslednji način

[inline-code-attrs-start title = 'Primer cURL zahtevka za posodobitev TenantUser'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za posodobitev TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Ko se spremeni email ali uporabniško ime, lahko to nastavite na true, da posodobite tudi uporabnikove komentarje. To bo podvojilo strošek kreditov. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za posodobitev TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Vključeno ob neuspehu. **/
    reason?: string
}
[inline-code-end]

---