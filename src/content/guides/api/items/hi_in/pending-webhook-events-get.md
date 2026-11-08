[api-resource-header-start name = 'PendingWebhookEvent'; route = 'GET /api/v1/pending-webhook-events'; creditsCost = 2; api-resource-header-end]

यह रूट `pendingWebhookEvents` पैरामीटर के अंतर्गत लंबित वेबहुक इवेंट्स की एक सूची लौटाता है।

यह API `skip` पैरामीटर द्वारा प्रदान किए गए पेजिनेशन का उपयोग करता है। PendingWebhookEvents `100` के पृष्ठों में लौटाए जाते हैं, जो `createdAt` के अनुसार नवीनतम पहले क्रम में होते हैं।

[inline-code-attrs-start title = 'PendingWebhookEvent cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsGetQueryParams {
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

[inline-code-attrs-start title = 'PendingWebhookEvent रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventsGetResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
    pendingWebhookEvents?: PendingWebhookEvent[]
}
[inline-code-end]