[api-resource-header-start name = 'TenantUser'; route = 'GET /api/v1/tenant-users'; creditsCost = 1; api-resource-header-end]

यह API पेजिनेशन का उपयोग करती है, जो `skip` क्वेरी पैरामीटर द्वारा प्रदान की जाती है। TenantUsers `100` के पृष्ठों में लौटाए जाते हैं, जो `signUpDate`, `username` और `id` के अनुसार क्रमबद्ध होते हैं।

लागत लौटाए गए tenant users की संख्या पर आधारित है, जो प्रति `10` लौटाए गए tenant users पर `1 credit` है।

[inline-code-attrs-start title = 'TenantUser cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-users?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** पेजिनेशन के लिए छोड़े जाने वाले tenant users की संख्या। **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUsersResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** विफलता पर शामिल। **/
    reason?: string
    tenantUsers?: TenantUser[]
}
[inline-code-end]