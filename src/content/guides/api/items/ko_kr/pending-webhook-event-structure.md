A `PendingWebhookEvent` 객체는 대기 중인 큐에 저장된 웹훅 이벤트를 나타냅니다.

`PendingWebhookEvent` 객체는 자동으로 생성되며 API를 통해 수동으로 생성할 수 없습니다. 또한 1년 후 만료됩니다.
큐에서 작업을 제거하는 삭제가 가능합니다.

이벤트 유형에는 여러 가지가 있습니다 - `eventType` (`OutboundSyncEventType`) 및 `type` (`OutboundSyncType`)을 확인하세요.

이 API의 일반적인 사용 사례 중 하나는 맞춤형 모니터링 구현입니다. 주기적으로 `/count` 엔드포인트를 호출하여 주어진 필터에 대한 미해결 수를 폴링할 수 있습니다.

`PendingWebhookEvent` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'PendingWebhookEvent 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum OutboundSyncEventType {
    Create: 0,
    Delete: 1,
    Update: 2
}

enum OutboundSyncType {
    /** WordPress 전용 동기화 작업. **/
    WP: 0,
    Webhook: 1
}

interface PendingWebhookEvent {
    id: string
    /** 이벤트와 연관된 댓글 ID. **/
    commentId: string
    /** 이벤트 시점의 댓글 객체. 2023년 11월부터 추가하기 시작했습니다. **/
    comment: Comment
    /** 댓글과 연결될 수 있는 외부 ID. **/
    externalId: string | null
    createdAt: Date
    tenantId: string
    attemptCount: number
    /** 첫 시도 전에 설정되며, 실패할 때마다 갱신됩니다. **/
    nextAttemptAt: Date
    /** 생성, 삭제, 또는 업데이트 이벤트인지 여부. **/
    eventType: OutboundSyncEventType
    /** 수행할 동기화 유형 (WordPress, API 호출 등). **/
    type: OutboundSyncType
    /** 댓글과 일치한 도메인. 이 도메인을 사용하여 API 키를 선택합니다. **/
    domain: string
    /** 발생한 마지막 오류. 이 타입은 구체적으로 정의되어 있지 않으며 발생한 내용을 그대로 덤프한 것입니다. 일반적으로 statusCode, body, headers 맵을 포함하는 객체를 담고 있습니다. **/
    lastError: object | null
}
[inline-code-end]