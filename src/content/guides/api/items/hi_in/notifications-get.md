[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

यह रूट `createdAt` के अनुसार क्रमबद्ध, नवीनतम पहले, 30 `Notification` ऑब्जेक्ट तक लौटाता है।

आप `userId` द्वारा फ़िल्टर कर सकते हैं। SSO के साथ, यूज़र आईडी `<tenant id>:<user id>` प्रारूप में होती है।

[inline-code-attrs-start title = 'यूज़र के लिए अपठित नोटिफिकेशन cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'नोटिफिकेशन रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** रिकॉर्ड छोड़कर पेजिनेट करें। **/
    skip?: number
    /** यूज़र द्वारा फ़िल्टर करें। **/
    userId?: string
    /** urlId द्वारा फ़िल्टर करें। **/
    urlId?: string
    /** स्रोत कमेंट द्वारा फ़िल्टर करें। **/
    fromCommentId?: string
    /** पढ़े/अपठित द्वारा फ़िल्टर करें। **/
    viewed?: 'true' | 'false'
    /** प्रकार द्वारा फ़िल्टर करें। **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'नोटिफिकेशन रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]