[api-resource-header-start name = 'EmailTemplate'; route = 'GET /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

यह API पेजिनेशन का उपयोग करता है, जो `page` क्वेरी पैरामीटर द्वारा प्रदान की जाती है। EmailTemplates `100` के पेजों में लौटाए जाते हैं, `createdAt` और फिर `id` के अनुसार क्रमबद्ध।

[inline-code-attrs-start title = 'EmailTemplate cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The page to fetch, starting with 0. **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    emailTemplates: EmailTemplate[]
}
[inline-code-end]