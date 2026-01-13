[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Deze route biedt de mogelijkheid om één `TenantUser` toe te voegen.

Het aanmaken van een `TenantUser` heeft de volgende beperkingen:

- Een `username` is verplicht.
- Een `email` is verplicht.
- De `signUpDate` mag niet in de toekomst liggen.
- De `locale` moet voorkomen in de lijst met [Ondersteunde Locales](https://github.com/FastComments/fastcomments-typescript/blob/main/src/constants.ts#L1).
- De `username` moet uniek zijn op heel FastComments.com. Als dit een probleem is, raden we aan in plaats daarvan SSO te gebruiken.
- De `email` moet uniek zijn op heel FastComments.com. Als dit een probleem is, raden we aan in plaats daarvan SSO te gebruiken.
- U mag niet meer tenant users aanmaken dan gedefinieerd onder `maxTenantUsers` in uw pakket. 

We kunnen een `TenantUser` als volgt aanmaken

[inline-code-attrs-start title = 'TenantUser Aanmaken cURL Voorbeeld'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "username": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Aanmaak Requeststructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Aanmaak Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'username-required' | 'email-required' | 'sign-up-date-in-future' | 'unsupported-locale' | 'unauthorized' | 'username-taken' | 'email-taken' | 'tenant-user-limit-reached'
    /** Opgenomen bij mislukking. **/
    reason?: string
    tenantUser?: TenantUser; // We geven de volledig aangemaakte TenantUser terug bij succes.
}
[inline-code-end]

---