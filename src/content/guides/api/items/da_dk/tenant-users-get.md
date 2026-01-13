[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

Dette API bruger paginering, leveret af `skip`-forespørgselsparameteren. TenantUsers returneres i sider af `100`, sorteret efter `signUpDate`, `username` og `id`.

Omkostningen er baseret på antallet af returnerede tenant-brugere og koster `1 kredit pr. 10` returnerede tenant-brugere.

[inline-code-attrs-start title = 'TenantUser cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenant users to skip for pagination. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]
