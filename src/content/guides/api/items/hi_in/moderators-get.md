[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

यह API पेजिनेशन का उपयोग करती है, जो `skip` क्वेरी पैरामीटर द्वारा प्रदान की जाती है। मॉडरेटर `100` के पृष्ठों में लौटाए जाते हैं, `createdAt` और `id` के अनुसार क्रमबद्ध।

लागत लौटाए गए मॉडरेटर की संख्या पर आधारित है, प्रत्येक `10 मॉडरेटर पर 1 क्रेडिट` की दर से।

[inline-code-attrs-start title = 'मॉडरेटर cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'मॉडरेटर रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** पेजिनेशन के लिए छोड़े जाने वाले मॉडरेटर की संख्या। **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'मॉडरेटर रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** विफलता पर शामिल। **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]