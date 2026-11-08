[api-resource-header-start name = 'NotificationCount'; route = 'GET /api/v1/notification-count/:user_id'; creditsCost = 1; api-resource-header-end]

यह रूट यूज़र आईडी द्वारा एकल `NotificationCount` लौटाता है। SSO के साथ, यूज़र आईडी `<tenant id>:<user id>` प्रारूप में होती है।

यदि कोई अपठित नोटिफिकेशन नहीं है, तो कोई `NotificationCount` नहीं होगा - इसलिए आपको 404 मिलेगा।

यह `notifications/count` से इस मायने में अलग है कि यह बहुत तेज़ है, लेकिन फ़िल्टरिंग की अनुमति नहीं देता।

[inline-code-attrs-start title = 'NotificationCount आईडी द्वारा cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notification-count/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
# -> {"status":"success","data":{"count":1,"createdAt":"2023-03-06T18:45:01.726Z","expireAt":"2024-03-06T01:25:01.726Z","id":"example"}}
[inline-code-end]

[inline-code-attrs-start title = 'NotificationCount रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'NotificationCount रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
    data?: NotificationCount
}
[inline-code-end]