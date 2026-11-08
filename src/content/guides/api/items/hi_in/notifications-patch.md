[api-resource-header-start name = 'Notification'; route = 'PATCH /api/v1/notifications/:id'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट `id` द्वारा एक `Notification` को अपडेट करने की क्षमता प्रदान करता है।

एक `Notification` को अपडेट करने की निम्नलिखित सीमाएँ हैं:

- आप केवल निम्नलिखित फ़ील्ड अपडेट कर सकते हैं:
  - `viewed`
  - `optedOut`

[inline-code-attrs-start title = 'Notification PATCH cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/notifications/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"viewed": false,
}'
[inline-code-end]

[inline-code-attrs-start title = 'Notification PATCH रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Notification PATCH रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface NotificationPatchResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]