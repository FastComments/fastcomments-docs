[api-resource-header-start name = 'TenantUser'; route = 'PUT /api/v1/tenant-users/:id'; creditsCost = 1; api-resource-header-end]

Deze route biedt de mogelijkheid om een enkele `TenantUser` te vervangen.

Het vervangen van een `TenantUser` heeft de volgende beperkingen:

- De `signUpDate` mag niet in de toekomst liggen.
- De `locale` moet voorkomen in de lijst van [Ondersteunde locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- De `username` moet uniek zijn binnen FastComments.com. Als dit een probleem is, raden we aan om in plaats daarvan SSO te gebruiken.
- De `email` moet uniek zijn binnen FastComments.com. Als dit een probleem is, raden we aan om in plaats daarvan SSO te gebruiken.
- U kunt de `tenantId` van een gebruiker niet bijwerken.

We kunnen een `TenantUser` als volgt aanmaken

[inline-code-attrs-start title = 'TenantUser Vervangen cURL Voorbeeld'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Vervangen Requeststructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Wanneer het e-mailadres of de gebruikersnaam wordt gewijzigd, kunt u dit op 'true' zetten om ook de opmerkingen van de gebruiker bij te werken. Hierdoor verdubbelt het aantal vereiste credits. **/
    updateComments: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Vervangen Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPutResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'unauthorized' | 'no-package' | 'invalid-package' | 'tenant-user-limit-reached' | 'user-does-not-exist'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]

---