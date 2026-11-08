[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

यह रूट एक ऑब्जेक्ट लौटाता है जिसमें `count` पैरामीटर के अंतर्गत सूचनाओं की संख्या होती है।

यह `/notification-count/` से धीमा है और क्रेडिट्स की लागत दोगुनी है, लेकिन यह अधिक आयामों पर फ़िल्टर करने की अनुमति देता है।

आप `/notifications` एंडपॉइंट के समान पैरामीटरों से फ़िल्टर कर सकते हैं, जैसे `userId`। SSO के साथ, यूज़र आईडी `<tenant id>:<user id>` प्रारूप में होती है।

[inline-code-attrs-start title = 'यूज़र के लिए अपठित सूचना संख्या cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'किसी विशिष्ट पेज के लिए यूज़र की अपठित सूचना संख्या cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'सूचना संख्या अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** यूज़र के अनुसार फ़िल्टर करें। **/
    userId?: string
    /** urlId के अनुसार फ़िल्टर करें। **/
    urlId?: string
    /** स्रोत कमेंट के अनुसार फ़िल्टर करें। **/
    fromCommentId?: string
    /** पढ़े/अपठित के अनुसार फ़िल्टर करें। **/
    viewed?: 'true' | 'false'
    /** प्रकार के अनुसार फ़िल्टर करें। **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'सूचना संख्या प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
    count?: number
}
[inline-code-end]