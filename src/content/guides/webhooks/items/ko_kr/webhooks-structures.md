The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### WebhookComment 객체 구조

##### The "Create" Event Structure  
"The create" event request body is a WebhookComment object.

##### The "Update" Event Structure  
"The update" event request body is a WebhookComment object.

##### The "Delete" Event Structure  
"The delete" event request body is a WebhookComment object.

2023년 11월 14일 변경  
Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Every key is always present in the body. When the comment has no value for a field the body carries `null` (or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

[inline-code-attrs-start title = 'WebhookComment 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 댓글의 ID. **/
    id: string
    /** 댓글 스레드를 식별하는 ID 또는 URL. 정규화됨. **/
    urlId: string
    /** 댓글이 남겨진 위치를 가리키는 URL. **/
    url: string | null
    /** 댓글을 남긴 사용자의 ID. SSO인 경우 테넌트 ID가 앞에 붙음. **/
    userId: string | null
    /** 댓글을 남긴 사용자의 이메일. **/
    commenterEmail: string | null
    /** 댓글 위젯에 표시되는 사용자의 이름. SSO인 경우 displayName일 수 있음. **/
    commenterName: string
    /** 원시 댓글 텍스트. **/
    comment: string
    /** 파싱된 후의 댓글 텍스트. **/
    commentHTML: string
    /** 댓글 외부 ID. **/
    externalId: string | null
    /** 상위 댓글의 ID. **/
    parentId: string | null
    /** 댓글이 작성된 UTC 날짜. **/
    date: UTC_ISO_DateString
    /** 투표의 총 카르마(업 - 다운). **/
    votes: number
    votesUp: number
    votesDown: number
    /** 사용자가 댓글을 달았을 때 로그인했거나, 댓글이 인증되었거나, 댓글이 남겨질 때 세션이 인증된 경우 true. **/
    verified: boolean
    /** 댓글이 인증된 UTC 날짜. **/
    verifiedDate: UTC_ISO_DateString | null
    /** 중재자가 댓글을 검토된 것으로 표시했는지 여부. **/
    reviewed: boolean
    /** 아바타의 위치 또는 base64 인코딩. SSO와 함께 전달된 값이 base64인 경우에만 base64가 됩니다. **/
    avatarSrc: string | null
    /** 댓글이 수동 또는 자동으로 스팸으로 표시되었는지 여부. **/
    isSpam: boolean
    /** 댓글이 자동으로 스팸으로 표시되었는지 여부. **/
    aiDeterminedSpam: boolean
    /** 댓글에 이미지가 있는지 여부. **/
    hasImages: boolean
    /** "가장 관련성 높은" 정렬 방향에서 댓글이 위치한 페이지 번호. **/
    pageNumber: number | null
    /** "오래된 순" 정렬 방향에서 댓글이 위치한 페이지 번호. **/
    pageNumberOF: number | null
    /** "최신 순" 정렬 방향에서 댓글이 위치한 페이지 번호. **/
    pageNumberNF: number | null
    /** 댓글이 자동 또는 수동으로 승인되었는지 여부. **/
    approved: boolean
    /** 댓글이 작성될 때 사용자의 로케일 코드(형식: en_us). **/
    locale: string | null
    /** 성공적으로 파싱된 댓글 내 @멘션. 없을 경우 비어 있음. **/
    mentions: CommentUserMention[]
    /** 댓글이 속한 도메인. **/
    domain: string | null
    /** 이 댓글과 연관된 중재 그룹 ID. 없을 경우 비어 있음. **/
    moderationGroupIds: string[]
    /** 이벤트가 생성될 당시 이 댓글 작성자 이름 옆에 표시된 배지. 없을 경우 비어 있음. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list has the following structure.

[inline-code-attrs-start title = 'Webhook 멘션 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 사용자 ID. SSO 사용자의 경우 테넌트 ID가 앞에 붙음. **/
    id: string
    /** 최종 @멘션 태그 텍스트, @ 기호 포함. **/
    tag: string
    /** 원본 @멘션 태그 텍스트, @ 기호 포함. **/
    rawTag: string
    /** 태그된 사용자의 유형. user = FastComments.com 계정. sso = SSOUser. **/
    type: 'user'|'sso'
    /** 사용자가 알림을 거부하더라도 이 값은 true로 설정됩니다. **/
    sent: boolean
}
[inline-code-end]

Each entry in the `badges` list has the following structure. See the [Badges guide](/guide-badges.html) for the badge types.

[inline-code-attrs-start title = 'Webhook 배지 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** 이 사용자에게 부여된 배지 ID. **/
    id: string
    /** 배지 유형. **/
    type: number
    /** 배지를 마우스오버할 때 표시되는 배지 설명. **/
    description: string
    /** 배지에 표시되는 텍스트. **/
    displayLabel?: string | null
    /** 배지가 이미지를 사용하는 경우 표시되는 이미지. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** 배지에 적용되는 추가 CSS 클래스. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP 메서드

You can configure the HTTP method for each webhook event type in the admin panel:

- **Create Event**: POST 또는 PUT (기본값: PUT)
- **Update Event**: POST 또는 PUT (기본값: PUT)
- **Delete Event**: DELETE, POST, 또는 PUT (기본값: DELETE)

모든 요청에 ID가 포함되므로, Create와 Update 작업은 기본적으로 (PUT) 멱등합니다. 동일한 Create 또는 Update 요청을 반복해도 귀하의 시스템에 중복 객체가 생성되지 않아야 합니다.

#### 요청 헤더

Each webhook request includes the following headers:

| 헤더 | 설명 |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | 귀하의 API 비밀키 |
| `X-FastComments-Timestamp` | 요청이 서명된 Unix 타임스탬프(초) |
| `X-FastComments-Signature` | HMAC-SHA256 서명 (`sha256=<hex>`) |

See [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.