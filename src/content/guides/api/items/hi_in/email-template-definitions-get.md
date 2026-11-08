[api-resource-header-start name = 'EmailTemplateDefinition'; route = 'GET /api/v1/email-templates/definitions'; creditsCost = 1; api-resource-header-end]

यह API सभी `EmailTemplateDefinition` ऑब्जेक्ट्स को प्राप्त करने की क्षमता प्रदान करता है।

[inline-code-attrs-start title = 'EmailTemplateDefinition GET cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/email-templates/definitions?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplateDefinition GET रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface GetEmailTemplateDefinitionsRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplateDefinition GET रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplateDefinition {
    name: string
    description: string
    /** सभी टेम्पलेट डोमेन से संबंधित नहीं होते। उदाहरण के लिए, कुछ एडमिन ईमेल कभी भी एक या अधिक डोमेन के संदर्भ में नहीं भेजे जाते। **/
    canBeDomainSpecific: boolean
    emailTemplateId: string
    defaultTestData: object
    defaultTranslationsByLocale: Record<string, Record<string, string>>
    defaultEJS: string
}

interface GetEmailTemplateDefinitionsResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** विफलता पर शामिल। **/
    reason?: string
    definitions?: EmailTemplateDefinition[]
}
[inline-code-end]