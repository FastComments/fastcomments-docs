[api-resource-header-start name = 'Me'; route = 'GET /api/v1/me'; creditsCost = 1; api-resource-header-end]

अनुरोध करने वाले क्रेडेंशियल का वर्णन करता है: वह टेनेंट जिसका यह है और, OAuth एक्सेस टोकन के मामले में, वह उपयोगकर्ता जिसने एप्लिकेशन को अधिकृत किया। इंटीग्रेशन इसका उपयोग कनेक्शन का परीक्षण करने और उसे लेबल करने के लिए करते हैं।

API कुंजी के साथ प्रतिक्रिया केवल टेनेंट की पहचान करती है। OAuth बेयरर टोकन के साथ यह अधिकृत करने वाले उपयोगकर्ता और प्रदान किए गए स्कोप भी ले जाता है।

[inline-code-attrs-start title = 'Me cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/me?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Me प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface MeResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** विफलता पर शामिल। **/
    reason?: string
    tenantId: string
    tenantName: string
    /** अनुरोध कैसे प्रमाणित हुआ। **/
    authType: 'api-key' | 'oauth'
    /** वे स्कोप जो क्रेडेंशियल के पास हैं। API कुंजियों के पास दोनों होते हैं। **/
    scopes: ('read' | 'write')[]
    /** केवल OAuth टोकन के लिए उपलब्ध। **/
    userId?: string
    username?: string
    email?: string
}
[inline-code-end]