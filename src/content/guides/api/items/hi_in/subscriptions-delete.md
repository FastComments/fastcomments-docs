[api-resource-header-start name = 'Subscription'; route = 'DELETE /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

यह रूट id द्वारा एकल `Subscription` ऑब्जेक्ट को डिलीट करता है।

[inline-code-attrs-start title = 'Subscription हटाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/subscriptions/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Subscription हटाने की Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Subscription हटाने की Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionDeleteResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]