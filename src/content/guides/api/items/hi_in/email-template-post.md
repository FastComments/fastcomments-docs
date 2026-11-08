[api-resource-header-start name = 'EmailTemplate'; route = 'POST /api/v1/email-templates'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट ईमेल टेम्पलेट बनाने की सुविधा प्रदान करता है।

नोट्स:

- आप एक ही डोमेन के साथ समान `emailTemplateId` वाले कई टेम्पलेट नहीं रख सकते।
- लेकिन आप एक वाइल्डकार्ड टेम्पलेट (`domain` = `*`) और उसी `emailTemplateId` के लिए एक डोमेन-विशिष्ट टेम्पलेट रख सकते हैं।
- `domain` निर्दिष्ट करना केवल तब प्रासंगिक है जब आपके पास अलग-अलग डोमेन हों, या आप परीक्षण के लिए विशिष्ट टेम्पलेट का उपयोग करना चाहते हों (`domain` को `localhost` आदि पर सेट करना)।
- यदि आप `domain` निर्दिष्ट करते हैं तो यह एक `DomainConfig` से मेल खाना चाहिए। त्रुटि होने पर वैध डोमेन की एक सूची प्रदान की जाती है।
- टेम्पलेट सिंटैक्स EJS है और इसे 500ms के टाइमआउट के साथ रेंडर किया जाता है। रेंडरिंग के लिए P99 <5ms है, इसलिए यदि आप 500ms तक पहुँचते हैं तो कुछ गड़बड़ है।
- **सहेजने के लिए आपका टेम्पलेट आपके दिए गए `testData` के साथ रेंडर होना चाहिए**। रेंडर त्रुटियों को एकत्र कर डैशबोर्ड में रिपोर्ट किया जाता है (जल्द ही API के माध्यम से उपलब्ध होगा)।

टेम्पलेट जोड़ने के लिए आवश्यक न्यूनतम डेटा इस प्रकार है:

[inline-code-attrs-start title = 'न्यूनतम EmailTemplate POST cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is an @mention notification! My name is <%= comment.commenterName %>."
}'
[inline-code-end]

आप प्रति-साइट टेम्पलेट रखना चाह सकते हैं, जिस स्थिति में आप `domain` परिभाषित करते हैं:

[inline-code-attrs-start title = 'EmailTemplate POST cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/email-templates?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "emailTemplateId": "comment-user-mention",
    "displayName": "I'm a custom template.",
    "ejs": "This is some email content!",
    "domain": "somespecificsite.com",
}'
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplatePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'EmailTemplate POST प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface EmailTemplatePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'unexpected-param' | 'invalid-email-template-id' | 'domain-invalid' | 'duplicate' | 'does-not-render';
    /** Included on failure. **/
    reason?: string
    /** The created template. **/
    emailTemplate?: EmailTemplate
}
[inline-code-end]