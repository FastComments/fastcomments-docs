[api-resource-header-start name = 'Comment'; route = 'GET /api/v1/domain-configs'; creditsCost = 1; api-resource-header-end]

यह API एक tenant के लिए सभी `DomainConfig` ऑब्जेक्ट्स प्राप्त करने की क्षमता प्रदान करता है।

[inline-code-attrs-start title = 'DomainConfig GET cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/domain-configs?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig GET रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface GetDomainConfigsRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig GET रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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