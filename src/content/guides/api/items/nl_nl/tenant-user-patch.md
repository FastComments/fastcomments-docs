[api-resource-header-start name = 'TenantUser'; route = 'PATCH /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Deze route biedt de mogelijkheid om een enkele `TenantUser` bij te werken.

Het bijwerken van een `TenantUser` heeft de volgende beperkingen:

- De `signUpDate` mag niet in de toekomst liggen.
- De `locale` moet in de lijst van [Ondersteunde locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1) staan.
- De `username` moet uniek zijn op heel FastComments.com. Als dit een probleem is, raden we aan in plaats daarvan SSO te gebruiken.
- De `email` moet uniek zijn op heel FastComments.com. Als dit een probleem is, raden we aan in plaats daarvan SSO te gebruiken.
- U kunt de `tenantId` van een gebruiker niet bijwerken.

We kunnen een `TenantUser` als volgt aanmaken

[inline-code-attrs-start title = 'TenantUser Update cURL Voorbeeld'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Update Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchQueryParams {
    tenantId: string
    API_KEY: string
    /** Wanneer e-mail of gebruikersnaam wordt gewijzigd, kunt u dit op 'true' zetten om ook de opmerkingen van de gebruiker bij te werken. Dit verdubbelt de kredietkosten. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Update Responsestructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPatchResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]

---