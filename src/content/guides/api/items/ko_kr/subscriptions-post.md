[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 `Subscription`을 생성하는 기능을 제공합니다. 사용자는 페이지당 하나의 구독만 가질 수 있다는 점에 유의하세요. 이는 중복되므로, 그리고
동일한 사용자와 동일한 페이지에 대해 둘 이상의 구독을 생성하려고 하면 오류가 발생합니다.

구독을 생성하면, 구독된 `urlId`의 루트에 새 댓글이 남겨질 때(댓글의 `parentId`가 `null`인 경우) `Notification` 객체가 생성됩니다.

[inline-code-attrs-start title = 'Subscription POST cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Subscription POST 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Subscription POST 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';  
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]