웹후크로 전송되는 유일한 구조는 아래 TypeScript에 설명된 WebhookComment 객체입니다.

#### WebhookComment 객체 구조

##### "Create" 이벤트 구조
"create" 이벤트 요청 본문은 WebhookComment 객체입니다.

##### "Update" 이벤트 구조
"update" 이벤트 요청 본문은 WebhookComment 객체입니다.

##### "Delete" 이벤트 구조
"delete" 이벤트 요청 본문은 WebhookComment 객체입니다.

    변경 — 2023년 11월 14일 기준
    이전에는 "delete" 이벤트 요청 본문에 댓글 id만 포함되어 있었습니다. 이제 삭제 시점의 전체 댓글을 포함합니다.


[inline-code-attrs-start title = 'WebhookComment 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 댓글의 id. **/
    id: string
    /** 댓글 스레드를 식별하는 id 또는 URL. 정규화됨. **/
    urlId: string
    /** 댓글이 남겨진 위치를 가리키는 URL. **/
    url?: string
    /** 댓글을 남긴 사용자의 id. SSO인 경우 tenant id가 접두사로 붙습니다. **/
    userId?: string
    /** 댓글을 남긴 사용자의 이메일. **/
    commenterEmail?: string
    /** 댓글 위젯에 표시되는 사용자 이름. SSO의 경우 displayName일 수 있음. **/
    commenterName: string
    /** 원시 댓글 텍스트. **/
    comment: string
    /** 파싱 후의 댓글 텍스트. **/
    commentHTML: string
    /** 외부 댓글 id. **/
    externalId?: string
    /** 부모 댓글의 id. **/
    parentId?: string | null
    /** 댓글이 작성된 UTC 날짜. **/
    date: UTC_ISO_DateString
    /** 투표의 합산 점수 (찬성 - 반대). **/
    votes: number
    votesUp: number
    votesDown: number
    /** 사용자가 댓글 작성 당시 로그인되어 있었거나, 댓글이 인증되었거나, 세션을 인증한 경우 true. **/
    verified: boolean
    /** 댓글이 인증된 날짜. **/
    verifiedDate?: number
    /** 모더레이터가 댓글을 검토됨으로 표시했는지 여부. **/
    reviewed: boolean
    /** 아바타의 위치 또는 base64 인코딩. SSO로 전달된 값이 base64였던 경우에만 base64로 제공됩니다. **/
    avatarSrc?: string
    /** 댓글이 수동으로 또는 자동으로 스팸으로 표시되었는지 여부. **/
    isSpam: boolean
    /** 댓글이 자동으로 스팸으로 표시되었는지 여부. **/
    aiDeterminedSpam: boolean
    /** 댓글에 이미지가 있는지 여부. **/
    hasImages: boolean
    /** 'Most Relevant' 정렬에서 댓글이 위치한 페이지 번호. **/
    pageNumber: number
    /** 'Oldest First' 정렬에서 댓글이 위치한 페이지 번호. **/
    pageNumberOF: number
    /** 'Newest First' 정렬에서 댓글이 위치한 페이지 번호. **/
    pageNumberNF: number
    /** 댓글이 자동으로 승인되었는지 또는 수동으로 승인되었는지 여부. **/
    approved: boolean
    /** 댓글 작성 당시 사용자의 로케일 코드(형식: en_us). **/
    locale: string
    /** 댓글에서 성공적으로 파싱된 @멘션 목록. **/
    mentions?: CommentUserMention[]
    /** 댓글이 작성된 도메인. **/
    domain?: string
    /** 이 댓글에 연결된 선택적 모더레이션 그룹 id 목록. **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Webhook Mentions 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 사용자 id. SSO 사용자의 경우 tenant id가 접두사로 붙습니다. **/
    id: string
    /** @ 기호를 포함한 최종 @멘션 태그 텍스트. **/
    tag: string
    /** @ 기호를 포함한 원본 @멘션 태그 텍스트. **/
    rawTag: string
    /** 태그된 사용자의 유형. user = FastComments.com 계정. sso = SSOUser. **/
    type: 'user'|'sso'
    /** 사용자가 알림 수신을 거부하더라도 이 값은 true로 설정됩니다. **/
    sent: boolean
}
[inline-code-end]

#### HTTP 메서드

관리자 패널에서 각 웹후크 이벤트 유형에 대한 HTTP 메서드를 구성할 수 있습니다:

- **Create Event**: POST 또는 PUT (기본값: PUT)
- **Update Event**: POST 또는 PUT (기본값: PUT)
- **Delete Event**: DELETE, POST 또는 PUT (기본값: DELETE)

모든 요청에 ID가 포함되어 있으므로 Create 및 Update 작업은 기본적으로 멱등합니다 (PUT). 동일한 Create 또는 Update 요청을 반복해도 귀측에서 중복 객체가 생성되지 않아야 합니다.

#### 요청 헤더

각 웹후크 요청에는 다음 헤더가 포함됩니다:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | 귀하의 API 시크릿 |
| `X-FastComments-Timestamp` | 요청이 서명된 시점의 Unix 타임스탬프(초) |
| `X-FastComments-Signature` | HMAC-SHA256 서명 (`sha256=<hex>`) |

HMAC 서명 검증에 대한 정보는 [보안 및 API 토큰](/guides/webhooks/webhooks-api-tokens)에서 확인하세요.

---