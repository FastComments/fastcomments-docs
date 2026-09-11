The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### WebhookComment 객체 구조

##### "Create" 이벤트 구조  
The "create" event request body is a WebhookComment object.

##### "Update" 이벤트 구조  
The "update" event request body is a WebhookComment object.

##### "Delete" 이벤트 구조  
The "delete" event request body is a WebhookComment object.

    2023년 11월 14일 변경
    이전에는 "delete" 이벤트 요청 본문에 댓글 ID만 포함되었습니다. 이제 삭제 시점의 전체 댓글이 포함됩니다.

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

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
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Webhook 멘션 객체'; type = 'typescript'; inline-code-attrs-end]
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

#### HTTP 메서드

You can configure the HTTP method for each webhook event type in the admin panel:

- **Create Event**: POST 또는 PUT (기본값: PUT)
- **Update Event**: POST 또는 PUT (기본값: PUT)
- **Delete Event**: DELETE, POST 또는 PUT (기본값: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

#### 요청 헤더

Each webhook request includes the following headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | 귀하의 API 비밀키 |
| `X-FastComments-Timestamp` | 요청이 서명된 시점의 Unix 타임스탬프(초) |
| `X-FastComments-Signature` | HMAC-SHA256 서명 (`sha256=<hex>`) |

See [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.