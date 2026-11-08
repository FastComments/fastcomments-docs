[api-resource-header-start name = 'DomainConfig'; route = 'PATCH /api/v1/domain-configs/:domain'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट केवल डोमेन और अपडेट करने के लिए एट्रिब्यूट निर्दिष्ट करके डोमेन कॉन्फ़िगरेशन अपडेट करने की क्षमता प्रदान करता है।

[inline-code-attrs-start title = 'DomainConfig PATCH cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/domain-configs/example.com?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"emailFromName": "Some New From Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig PATCH रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig PATCH रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface DomainConfigPatchResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'missing-domain' | 'domain-does-not-exist' | 'update-would-create-duplicate' | 'domain-too-long' | 'domain-invalid';  
    /** विफलता पर शामिल। **/
    reason?: string
    /** अपडेट की गई कॉन्फ़िगरेशन। **/
    configuration?: DomainConfig
}
[inline-code-end]