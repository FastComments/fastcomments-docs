[api-resource-header-start name = 'SSOUser'; route = 'GET /api/v1/sso-users/by-email/:email'; creditsCost = 1; api-resource-header-end]

Ова рута враћа једног SSO корисника на основу њиховог имејла.

[inline-code-attrs-start title = 'SSOUser по имејлу - пример cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/sso-users/by-email/someone@somewhere.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser - структура захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserRequestByEmailQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser - структура одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserByEmailResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-email' | 'user-does-not-exist'
    /** Укључено у случају неуспеха. **/
    reason?: string
    user: SSOUser
}
[inline-code-end]