웹훅을 통해 전송되는 유일한 구조는 WebhookComment 객체이며, 아래 TypeScript로 정리되어 있습니다.

#### WebhookComment 객체 구조

##### "Create" 이벤트 구조
"create" 이벤트의 요청 본문은 WebhookComment 객체입니다.

##### "Update" 이벤트 구조
"update" 이벤트의 요청 본문은 WebhookComment 객체입니다.

##### "Delete" 이벤트 구조
"delete" 이벤트의 요청 본문은 WebhookComment 객체입니다.

    2023년 11월 14일 변경 사항
    이전에는 "delete" 이벤트의 요청 본문에 댓글 id만 포함되었습니다. 이제 삭제 시점의 전체 댓글이 포함됩니다.


모든 키는 본문에 항상 존재합니다. 댓글에 특정 필드의 값이 없으면 본문에는 `null`(불리언의 경우 `false`, 리스트의 경우 `[]`)이 담기므로, 전달되는 데이터의 형태는 댓글마다 달라지지 않습니다.

[inline-code-attrs-start title = 'WebhookComment 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** The id of the comment. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** The URL that points to where the comment was left. **/
    url: string | null
    /** The user id that left the comment. If SSO, prefixed with tenant id. **/
    userId: string | null
    /** The email of the user left the comment. **/
    commenterEmail: string | null
    /** The name of the user that shows in the comment widget. With SSO, can be displayName. **/
    commenterName: string
    /** Raw comment text. **/
    comment: string
    /** Comment text after parsing. **/
    commentHTML: string
    /** Comment external id. **/
    externalId: string | null
    /** The id of the parent comment. **/
    parentId: string | null
    /** The UTC date when the comment was left. **/
    date: UTC_ISO_DateString
    /** Combined karma (up - down) of votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True if the user was logged in when they commented, or their verified the comment, or if they verified their session when the comment was left. **/
    verified: boolean
    /** The UTC date when the comment was verified. **/
    verifiedDate: UTC_ISO_DateString | null
    /** If a moderator marked the comment reviewed. **/
    reviewed: boolean
    /** The location, or base64 encoding, of the avatar. Will only be base64 if that was the value passed with SSO. **/
    avatarSrc: string | null
    /** Was the comment manually or automatically marked as spam? **/
    isSpam: boolean
    /** Was the comment automatically marked as spam? **/
    aiDeterminedSpam: boolean
    /** Are there images in the comment? **/
    hasImages: boolean
    /** The page number the comment is on for the "Most Relevant" sort direction. **/
    pageNumber: number | null
    /** The page number the comment is on for the "Oldest First" sort direction. **/
    pageNumberOF: number | null
    /** The page number the comment is on for the "Newest First" sort direction. **/
    pageNumberNF: number | null
    /** Was the comment approved automatically or manually? **/
    approved: boolean
    /** The locale code (format: en_us) of the user when the comment was written. **/
    locale: string | null
    /** The @mentions written in the comment that were successfully parsed. Empty when there are none. **/
    mentions: CommentUserMention[]
    /** The domain the comment is from. **/
    domain: string | null
    /** The moderation group ids associated with this comment. Empty when there are none. **/
    moderationGroupIds: string[]
    /** The badges shown next to the commenter's name on this comment, as of when the event was created. Empty when there are none. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

댓글에서 사용자가 태그되면 해당 정보는 `mentions`라는 리스트에 저장됩니다. 이 리스트의 각 객체는 다음과 같은 구조를 가집니다.

[inline-code-attrs-start title = '웹훅 Mentions 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    sent: boolean
}
[inline-code-end]

`badges` 리스트의 각 항목은 다음과 같은 구조를 가집니다. 배지 유형에 대해서는 [배지 가이드](/guide-badges.html)를 참조하세요.

[inline-code-attrs-start title = '웹훅 배지 객체'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** The id of the badge awarded to this user. **/
    id: string
    /** The badge type. **/
    type: number
    /** The badge description, shown when hovering over the badge. **/
    description: string
    /** The text shown on the badge. **/
    displayLabel?: string | null
    /** The image shown for the badge, if it uses one. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** An extra CSS class applied to the badge. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP 메서드

관리자 패널에서 각 웹훅 이벤트 유형에 대한 HTTP 메서드를 설정할 수 있습니다:

- **Create 이벤트**: POST 또는 PUT (기본값: PUT)
- **Update 이벤트**: POST 또는 PUT (기본값: PUT)
- **Delete 이벤트**: DELETE, POST 또는 PUT (기본값: DELETE)

모든 요청에 ID가 포함되므로, Create 및 Update 작업은 기본적으로(PUT) 멱등성을 가집니다. 동일한 Create 또는 Update 요청을 반복해도 사용자 측에 중복 객체가 생성되지 않아야 합니다.

#### 요청 헤더

각 웹훅 요청에는 다음 헤더가 포함됩니다:

| 헤더 | 설명 |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | 귀하의 API Secret |
| `X-FastComments-Timestamp` | 요청이 서명된 Unix 타임스탬프(초) |
| `X-FastComments-Signature` | HMAC-SHA256 서명 (`sha256=<hex>`) |

HMAC 서명 검증에 대한 정보는 [보안 및 API 토큰](/guide-webhooks.html#webhooks-api-tokens)을 참조하세요.