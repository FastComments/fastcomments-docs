[api-resource-header-start name = 'DomainConfig'; route = 'DELETE /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

यह रूट id द्वारा एकल `DomainConfig` को हटाने की सुविधा प्रदान करता है।

- नोट: `DomainConfig` को हटाने से उस डोमेन का FastComments उपयोग करने का अधिकार समाप्त हो जाएगा।
- नोट: UI के माध्यम से डोमेन को दोबारा जोड़ने पर ऑब्जेक्ट फिर से बन जाएगा (केवल `domain` पॉपुलेटेड के साथ)।

[inline-code-attrs-start title = 'DomainConfig हटाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig हटाने के रिक्वेस्ट की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig हटाने के रिस्पॉन्स की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-domain' | 'domain-config-does-not-exist'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]