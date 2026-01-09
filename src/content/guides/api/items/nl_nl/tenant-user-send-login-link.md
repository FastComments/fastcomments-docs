[api-resource-header-start name = 'TenantUser'; route = 'POST /api/v1/tenant-users/:id/send-login-link'; creditsCost = 10; api-resource-header-end]

Deze route biedt de mogelijkheid om een aanmeldlink te verzenden naar een enkele `TenantUser`.

Handig bij het batchgewijs aanmaken van gebruikers, zodat je ze niet hoeft uit te leggen hoe ze kunnen inloggen op FastComments.com. Dit stuurt hen eenvoudig een "magische link" om in te loggen die vervalt na `30 days`.

De volgende beperkingen gelden voor het verzenden van een aanmeldlink naar een `TenantUser`:
- De `TenantUser` moet al bestaan.
- Je moet toegang hebben om de `Tenant` te beheren waartoe de `TenantUser` behoort.

We kunnen een aanmeldlink naar een `TenantUser` verzenden als volgt:

[inline-code-attrs-start title = 'TenantUser Login Link cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz/send-login-link?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

Dit zal een e-mail sturen zoals `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'TenantUser Login Link Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Login Link Antwoordstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserPostResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]

---