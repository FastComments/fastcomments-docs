[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

이 라우트는 `createdAt`을 기준으로 최신 순으로 정렬된 최대 30개의 `Notification` 객체를 반환합니다.

`userId`로 필터링할 수 있습니다. SSO의 경우 사용자 ID는 `<tenant id>:<user id>` 형식입니다.

[inline-code-attrs-start title = '사용자 읽지 않은 알림 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = '알림 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** 레코드를 건너뛰어 페이지네이션합니다. **/
    skip?: number
    /** 사용자별 필터. **/
    userId?: string
    /** urlId로 필터링. **/
    urlId?: string
    /** 원본 댓글로 필터링. **/
    fromCommentId?: string
    /** 읽음/읽지 않음으로 필터링. **/
    viewed?: 'true' | 'false'
    /** 유형으로 필터링. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = '알림 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** 실패 시 포함됩니다. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]

---