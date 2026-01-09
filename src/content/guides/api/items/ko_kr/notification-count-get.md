[api-resource-header-start name = 'NotificationCount'; route = 'GET /api/v1/notification-count/:user_id'; creditsCost = 1; api-resource-header-end]

이 라우트는 사용자 ID로 단일 `NotificationCount`를 반환합니다. SSO를 사용하는 경우 사용자 ID 형식은 `<tenant id>:<user id>`입니다.

읽지 않은 알림이 없으면 `NotificationCount`가 없으므로 404를 받게 됩니다.

`notifications/count`와 달리 훨씬 빠르지만 필터링은 허용하지 않습니다.

[inline-code-attrs-start title = 'NotificationCount ID별 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notification-count/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
# -> {"status":"success","data":{"count":1,"createdAt":"2023-03-06T18:45:01.726Z","expireAt":"2024-03-06T01:25:01.726Z","id":"example"}}
[inline-code-end]

[inline-code-attrs-start title = 'NotificationCount 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'NotificationCount 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'not-found'
    /** 실패 시 포함됩니다. **/
    reason?: string
    data?: NotificationCount
}
[inline-code-end]