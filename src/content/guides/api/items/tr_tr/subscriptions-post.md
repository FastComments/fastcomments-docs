[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

This API endpoint provides the ability to create a `Subscription`. Note that a user may only have one subscription per page, as more is redundant, and trying
to create more than one subscription for the same user for the same page will result in an error.

Creating a subscription will result in `Notification` objects being created when a new comment is left on the root of the subscribed `urlId` (when comment `parentId` is `null`).

[inline-code-attrs-start title = 'Abonelik POST cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "userId": "tenantId:test-user-id",
    "urlId": "some-page-id-or-url",
    "url": "https://example.com/page",
    "pageTitle": "Some Example Page!"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Abonelik POST İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Abonelik POST Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Hata durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** Hata durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]