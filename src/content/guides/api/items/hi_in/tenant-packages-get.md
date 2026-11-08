[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

यह API पेजिनेशन का उपयोग करती है, जो `skip` क्वेरी पैरामीटर द्वारा प्रदान की जाती है। TenantPackages `100` के पृष्ठों में लौटाए जाते हैं, `createdAt` और `id` के अनुसार क्रमबद्ध।

लागत लौटाए गए टेनेंट पैकेजों की संख्या पर आधारित है, जिसमें प्रत्येक `10` टेनेंट पैकेजों पर `1 क्रेडिट` लगता है।

[inline-code-attrs-start title = 'TenantPackage cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** पेजिनेशन के लिए छोड़े जाने वाले टेनेंट पैकेजों की संख्या। **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** विफलता पर शामिल। **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]