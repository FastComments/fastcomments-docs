The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### The WebhookComment Object Structure

##### The "Create" Event Structure  
The "create" event request body is a WebhookComment object.

##### The "Update" Event Structure  
The "update" event request body is a WebhookComment object.

##### The "Delete" Event Structure  
The "delete" event request body is a WebhookComment object.

    Change as of Nov 14th 2023  
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

Every key is always present in the body. When the comment has no value for a field the body carries `null`  
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

[inline-code-attrs-start title = 'WebhookComment 对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 评论的 ID。 **/
    id: string
    /** 标识评论线程的 ID 或 URL，已规范化。 **/
    urlId: string
    /** 指向留下评论的页面的 URL。 **/
    url: string | null
    /** 留下评论的用户 ID。如果是 SSO，则前缀为租户 ID。 **/
    userId: string | null
    /** 留下评论的用户的电子邮件。 **/
    commenterEmail: string | null
    /** 在评论小部件中显示的用户名称。使用 SSO 时，可为 displayName。 **/
    commenterName: string
    /** 原始评论文本。 **/
    comment: string
    /** 解析后的评论 HTML。 **/
    commentHTML: string
    /** 评论的外部 ID。 **/
    externalId: string | null
    /** 父评论的 ID。 **/
    parentId: string | null
    /** 评论留下时的 UTC 日期。 **/
    date: UTC_ISO_DateString
    /** 投票的综合 karma（赞 - 踩）。 **/
    votes: number
    votesUp: number
    votesDown: number
    /** 如果用户在发表评论时已登录，或已验证评论，或在评论留下时验证了会话，则为 true。 **/
    verified: boolean
    /** 评论验证的 UTC 日期。 **/
    verifiedDate: UTC_ISO_DateString | null
    /** 如果版主标记了评论已审阅。 **/
    reviewed: boolean
    /** 头像的位置信息或 base64 编码。仅在 SSO 传递该值时为 base64。 **/
    avatarSrc: string | null
    /** 评论是手动还是自动标记为垃圾评论？ **/
    isSpam: boolean
    /** 评论是否被自动标记为垃圾评论？ **/
    aiDeterminedSpam: boolean
    /** 评论中是否包含图片？ **/
    hasImages: boolean
    /** “最相关”排序方向下评论所在的页码。 **/
    pageNumber: number | null
    /** “最旧优先”排序方向下评论所在的页码。 **/
    pageNumberOF: number | null
    /** “最新优先”排序方向下评论所在的页码。 **/
    pageNumberNF: number | null
    /** 评论是自动还是手动批准的？ **/
    approved: boolean
    /** 评论撰写时用户的语言代码（格式：en_us）。 **/
    locale: string | null
    /** 评论中成功解析的 @提及，若无则为空。 **/
    mentions: CommentUserMention[]
    /** 评论所属的域名。 **/
    domain: string | null
    /** 与此评论关联的审核组 ID，若无则为空。 **/
    moderationGroupIds: string[]
    /** 事件创建时显示在评论者名称旁的徽章，若无则为空。 **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Webhook 提及对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 用户 ID。对于 SSO 用户，会在前面加上租户 ID。 **/
    id: string
    /** 最终的 @提及标签文本，包括 @ 符号。 **/
    tag: string
    /** 原始的 @提及标签文本，包括 @ 符号。 **/
    rawTag: string
    /** 被标记的用户类型。user = FastComments.com 账户，sso = SSOUser。 **/
    type: 'user'|'sso'
    /** 即使用户选择不接收通知，此字段仍为 true。 **/
    sent: boolean
}
[inline-code-end]

Each entry in the `badges` list has the following structure. See the [Badges guide](/guide-badges.html) for the badge types.

[inline-code-attrs-start title = 'Webhook 徽章对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** 授予此用户的徽章 ID。 **/
    id: string
    /** 徽章类型。 **/
    type: number
    /** 当鼠标悬停在徽章上时显示的描述。 **/
    description: string
    /** 徽章上显示的文字。 **/
    displayLabel?: string | null
    /** 徽章使用的图片（如果有）。 **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** 应用于徽章的额外 CSS 类。 **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP Methods

You can configure the HTTP method for each webhook event type in the admin panel:

- **Create Event**: POST or PUT (default: PUT)
- **Update Event**: POST or PUT (default: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE)

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

#### Request Headers

Each webhook request includes the following headers:

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Your API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Signature` | HMAC-SHA256 signature (`sha256=<hex>`) |

See [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.