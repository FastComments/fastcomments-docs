[api-resource-header-start name = 'NotificationCount'; route = 'DELETE /api/v1/notification-count/:user_id'; creditsCost = 1; api-resource-header-end]

यह रूट यूज़र आईडी द्वारा एकल `NotificationCount` को डिलीट करता है। SSO के साथ, यूज़र आईडी `<tenant id>:<user id>` फ़ॉर्मैट में होती है।

यह यूज़र की अपठित नोटिफिकेशन गिनती को साफ़ कर देगा (कमेंट विजेट में लाल घंटी फीकी पड़ जाएगी और गिनती हट जाएगी)।

[inline-code-attrs-start title = 'NotificationCount डिलीट करने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/notification-count/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
# -> {"status":"success"}
[inline-code-end]

[inline-code-attrs-start title = 'NotificationCount हटाने के रिक्वेस्ट की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'NotificationCount हटाने के रिस्पॉन्स की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]