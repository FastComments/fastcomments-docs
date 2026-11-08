[api-resource-header-start name = 'DomainConfig'; route = 'GET /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

अलग-अलग DomainConfigs को उनके संबंधित `domain` के आधार पर प्राप्त (fetch) किया जा सकता है।

[inline-code-attrs-start title = 'डोमेन द्वारा डोमेन कॉन्फ़िग cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'डोमेन द्वारा डोमेन कॉन्फ़िग रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigsByDomainRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'डोमेन द्वारा डोमेन कॉन्फ़िग रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'internal' | 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-domain' | 'update-would-create-duplicate' | 'domain-does-not-exist'
    /** Included on failure. **/
    reason?: string
    configuration?: DomainConfig | null
}
[inline-code-end]