[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 댓글을 생성하는 기능을 제공합니다.

일반적인 사용 사례는 맞춤 UI, 통합 또는 가져오기입니다.

참고:

- 원하면 이 API는 댓글 위젯을 "라이브"로 업데이트할 수 있습니다 (이 경우 `creditsCost`가 `1`에서 `2`로 증가합니다).
- 이메일이 제공되면 이 API는 자동으로 사용자 객체를 시스템에 생성합니다.
- 다른 이메일을 사용하지만 동일한 사용자 이름을 가진 두 댓글을 저장하려고 하면 두 번째 댓글에 대해 오류가 발생합니다.
- 만약 `parentId`를 지정하고 자식 댓글의 `notificationSentForParent`가 false인 경우, **부모 댓글에 대한 알림을 전송합니다**. 이 작업은 매 시간 수행됩니다(전송되는 이메일 수를 줄이기 위해 알림을 일괄 처리합니다).
- 사용자를 생성할 때 환영 이메일을 보내거나 댓글 확인 이메일을 보내려면 쿼리 매개변수에서 `sendEmails`를 `true`로 설정하세요.
- 이 API로 생성된 댓글은 관리자 앱의 Analytics 및 Moderation 페이지에 표시됩니다.
- 설정이 켜져 있으면 "bad words"는 댓글 작성자 이름과 댓글 내용에서 여전히 마스킹됩니다.
- 원하면 이 API로 생성된 댓글은 스팸 검사를 받을 수 있습니다.
- Customization Rule 관리자 페이지에서 구성된 최대 댓글 길이 같은 설정은 여기에도 적용됩니다.

댓글 위젯에 표시되기 위해 제출에 필요한 최소 데이터는 다음과 같습니다:

[inline-code-attrs-start title = '최소 댓글 POST cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

더 현실적인 요청 예시는 다음과 같습니다:

[inline-code-attrs-start title = '댓글 POST cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = '댓글 POST 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** 댓글이 동일한 urlId를 가진 댓글 위젯 인스턴스를 보는 사용자에게 "라이브"로 표시될지 여부. 참고: 크레딧 비용이 1에서 2로 두 배가 됩니다. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = '댓글 POST 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 생성된 댓글. **/
    comment?: Comment
    /** 관련된 사용자로, 기존에 존재했을 수도 있고 없었을 수도 있습니다. **/
    user?: User
}
[inline-code-end]

---