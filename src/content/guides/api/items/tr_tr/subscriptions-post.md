[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Bu API uç noktası bir `Subscription` oluşturma yeteneği sağlar. Bir kullanıcının sayfa başına yalnızca bir aboneliği olabilir, çünkü fazlası gereksizdir; aynı kullanıcı için aynı sayfaya birden fazla abonelik oluşturulmaya çalışılması bir hatayla sonuçlanacaktır.

Bir abonelik oluşturmak, abone olunan `urlId`'nin köküne yeni bir yorum bırakıldığında (yorum `parentId`'si `null` olduğunda) `Notification` nesnelerinin oluşturulmasına neden olur.

[inline-code-attrs-start title = 'Subscription POST cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Subscription POST İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Subscription POST Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]