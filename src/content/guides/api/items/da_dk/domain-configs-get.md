[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/domain-configs'; creditsCost = 1; api-resource-header-end]

Denne API giver mulighed for at hente alle `DomainConfig`-objekter for en tenant.

[inline-code-attrs-start title = 'DomainConfig GET cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/domain-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig GET Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface GetDomainConfigsRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig GET Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface GetDomainConfigsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    /** The configurations! **/
    configurations: DomainConfig[] | null
}
[inline-code-end]
