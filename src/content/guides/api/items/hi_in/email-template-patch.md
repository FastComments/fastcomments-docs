[api-resource-header-start name = 'EmailTemplate'; route = 'PATCH /api/v1/email-templates/:id'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट केवल id और अपडेट करने के लिए विशेषताओं (attributes) को निर्दिष्ट करके ईमेल टेम्पलेट अपडेट करने की क्षमता प्रदान करता है।

ध्यान दें कि टेम्पलेट बनाने के लिए सभी समान सत्यापन (validations) यहाँ भी लागू होते हैं, उदाहरण के लिए:

- टेम्पलेट रेंडर होना चाहिए। यह प्रत्येक अपडेट के साथ जाँचा जाता है।
- एक ही डोमेन के लिए डुप्लिकेट टेम्पलेट नहीं हो सकते (अन्यथा एक को चुपचाप अनदेखा कर दिया जाएगा)।

[inline-code-attrs-start title = 'EmailTemplate PATCH cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/email-templates/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"displayName": "Some New Name",
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate PATCH रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePatchResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'not-found' | 'unexpected-param' | 'invalid-email-template-id' | 'unauthorized' | 'domain-invalid' | 'duplicate' | 'does-not-render';  
    /** विफलता पर शामिल। **/
    reason?: string
    /** अपडेट किया गया ईमेल टेम्पलेट। **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]