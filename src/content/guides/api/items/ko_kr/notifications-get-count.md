[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

이 라우트는 `count` 매개변수에 알림 수를 담은 객체를 반환합니다.

이 라우트는 `/notification-count/`보다 느리고 크레딧 비용이 두 배이지만 더 많은 차원으로 필터링할 수 있습니다.

`/notifications` 엔드포인트와 동일한 매개변수(예: `userId`)로 필터링할 수 있습니다. SSO를 사용하는 경우 user id는 `<tenant id>:<user id>` 형식입니다.

[inline-code-attrs-start title = '사용자 읽지 않은 알림 수 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = '특정 페이지의 사용자 읽지 않은 알림 수 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = '알림 수 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** 사용자로 필터링. **/
    userId?: string
    /** urlId로 필터링. **/
    urlId?: string
    /** 출처 댓글로 필터링. **/
    fromCommentId?: string
    /** 읽음 여부로 필터링. **/
    viewed?: 'true' | 'false'
    /** 타입으로 필터링. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = '알림 수 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 실패 시 포함됩니다. **/
    reason?: string
    count?: number
}
[inline-code-end]