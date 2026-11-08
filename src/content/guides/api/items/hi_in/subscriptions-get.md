[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

यह रूट `createdAt` के अनुसार क्रमबद्ध, नवीनतम पहले, अधिकतम 30 `Subscription` ऑब्जेक्ट लौटाता है।

आप `userId` द्वारा फ़िल्टर कर सकते हैं। SSO के साथ, यूज़र आईडी `<tenant id>:<user id>` प्रारूप में होती है।

[inline-code-attrs-start title = 'यूज़र के लिए सब्सक्रिप्शन cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'सब्सक्रिप्शन रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** रिकॉर्ड छोड़कर पेजिनेट करें। **/
    skip?: number
    /** यूज़र द्वारा फ़िल्टर करें। **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'सब्सक्रिप्शन रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]