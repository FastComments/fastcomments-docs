[api-resource-header-start name = 'PendingWebhookEvent'; route = 'DELETE /api/v1/pending-webhook-events/:id'; creditsCost = 1; api-resource-header-end]

यह रूट एक सिंगल `PendingWebhookEvent` को डिलीट करने की अनुमति देता है।

यदि आपको बल्क डिलीट करने की आवश्यकता है, तो GET API को पेजिनेशन के साथ कॉल करें और फिर इस API को क्रमिक रूप से कॉल करें।

[inline-code-attrs-start title = 'PendingWebhookEvent डिलीट करने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/pending-webhook-events/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent डिलीट रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PendingWebhookEvent डिलीट रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PendingWebhookEventDeleteResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]