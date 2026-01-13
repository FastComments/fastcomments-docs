[api-resource-header-start name = 'SSOUser'; route = 'GET /api/v1/sso-users/by-email/:email'; creditsCost = 1; api-resource-header-end]

Deze route retourneert een enkele SSO-gebruiker op basis van hun e-mailadres.

[inline-code-attrs-start title = 'SSOUser per e-mail cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/sso-users/by-email/someone@somewhere.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserRequestByEmailQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserByEmailResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-email' | 'user-does-not-exist'
    /** Opgenomen bij mislukking. **/
    reason?: string
    user: SSOUser
}
[inline-code-end]

---