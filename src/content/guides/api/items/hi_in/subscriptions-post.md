[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट एक `Subscription` बनाने की क्षमता प्रदान करता है। ध्यान दें कि एक उपयोगकर्ता के पास प्रति पेज केवल एक सब्सक्रिप्शन हो सकता है, क्योंकि अधिक होना निरर्थक है, और उसी उपयोगकर्ता के लिए उसी पेज पर एक से अधिक सब्सक्रिप्शन बनाने की कोशिश करने पर त्रुटि होगी।

सब्सक्रिप्शन बनाने पर, सब्सक्राइब किए गए `urlId` के रूट पर नया कमेंट आने पर (जब कमेंट का `parentId` `null` हो) `Notification` ऑब्जेक्ट्स बनाए जाएंगे।

[inline-code-attrs-start title = 'Subscription POST cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "userId": "tenantId:test-user-id",
    "urlId": "some-page-id-or-url",
    "url": "https://example.com/page",
    "pageTitle": "Some Example Page!"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Subscription POST रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Subscription POST रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]