[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events/count'; creditsCost = 2; api-resource-header-end]

यह रूट एक ऑब्जेक्ट लौटाता है जिसमें `count` पैरामीटर के अंतर्गत लंबित वेबहुक इवेंट्स की संख्या होती है।

आप `/pending-webhook-events` एंडपॉइंट के समान पैरामीटर्स से फ़िल्टर कर सकते हैं।

[inline-code-attrs-start title = 'PendingWebhookEvent Count cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events/count?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent Count रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountQueryParams {
    tenantId: string
    API_KEY: string
    commentId?: string
    externalId?: string
    eventType?: OutboundSyncEventType
    type?: OutboundSyncType
    domain?: string
    /** निर्दिष्ट संख्या से अधिक प्रयास संख्या वाले इवेंट्स के लिए क्वेरी। **/
    attemptCountGT?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent Count रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsCountResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
    count?: number
}
[inline-code-end]