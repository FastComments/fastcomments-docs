[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

이 경로는 `createdAt` 기준으로 정렬된 최신순으로 최대 30개의 `Subscription` 객체를 반환합니다.

`userId`로 필터링할 수 있습니다. SSO를 사용하는 경우 사용자 ID는 `<tenant id>:<user id>` 형식입니다.

[inline-code-attrs-start title = '사용자용 구독 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = '구독 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** 레코드를 건너뛰어 페이지네이션합니다. **/
    skip?: number
    /** 사용자로 필터링합니다. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = '구독 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 실패 시 포함됩니다. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]

---