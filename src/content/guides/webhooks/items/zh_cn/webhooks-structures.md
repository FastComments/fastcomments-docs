The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### WebhookComment 对象结构

##### “Create” 事件结构  
“create” 事件的请求体是一个 WebhookComment 对象。

##### “Update” 事件结构  
“update” 事件的请求体是一个 WebhookComment 对象。

##### “Delete” 事件结构  
“delete” 事件的请求体是一个 WebhookComment 对象。

    更改于 2023 年 11 月 14 日
    之前，“delete” 事件的请求体仅包含评论 ID。现在它在删除时包含完整的评论。

Every key is always present in the body. When the comment has no value for a field the body carries `null`
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

[inline-code-attrs-start title = 'WebhookComment 对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 评论的 ID。 **/
    id: string
    /** 标识评论线程的 ID 或 URL。已规范化。 **/
    urlId: string
    /** 指向评论所在位置的 URL。 **/
    url: string | null
    /** 留下评论的用户 ID。如果是 SSO，则前缀为租户 ID。 **/
    userId: string | null
    /** 留下评论的用户的电子邮件。 **/
    commenterEmail: string | null
    /** 在评论小部件中显示的用户名称。对于 SSO，可能是 displayName。 **/
    commenterName: string
    /** 原始评论文本。 **/
    comment: string
    /** 解析后的评论文本。 **/
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
    /** 如果用户在评论时已登录，或已验证评论，或在评论时验证了其会话，则为 true。 **/
    verified: boolean
    /** 评论被验证的 UTC 日期。 **/
    verifiedDate: UTC_ISO_DateString | null
    /** 如果版主将评论标记为已审阅。 **/
    reviewed: boolean
    /** 头像的位置或 base64 编码。如果是通过 SSO 传递的值，则仅为 base64。 **/
    avatarSrc: string | null
    /** 评论是手动还是自动标记为垃圾信息？ **/
    isSpam: boolean
    /** 评论是否被自动标记为垃圾信息？ **/
    aiDeterminedSpam: boolean
    /** 评论中是否包含图片？ **/
    hasImages: boolean
    /** 评论在“最相关”排序方向所在的页码。 **/
    pageNumber: number | null
    /** 评论在“最旧优先”排序方向所在的页码。 **/
    pageNumberOF: number | null
    /** 评论在“最新优先”排序方向所在的页码。 **/
    pageNumberNF: number | null
    /** 评论是自动还是手动批准的？ **/
    approved: boolean
    /** 用户在撰写评论时的语言区域代码（格式：en_us）。 **/
    locale: string | null
    /** 评论中成功解析的 @提及。若无则为空。 **/
    mentions: CommentUserMention[]
    /** 评论所属的域名。 **/
    domain: string | null
    /** 与此评论关联的审核组 ID。若无则为空。 **/
    moderationGroupIds: string[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list has the following structure.

[inline-code-attrs-start title = 'Webhook 提及对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 用户 ID。对于 SSO 用户，会在前面加上租户 ID。 **/
    id: string
    /** 最终的 @提及标签文本，包括 @ 符号。 **/
    tag: string
    /** 原始的 @提及标签文本，包括 @ 符号。 **/
    rawTag: string
    /** 被标记的用户类型。user = FastComments.com 账户。sso = SSOUser。 **/
    type: 'user'|'sso'
    /** 即使用户选择不接收通知，此字段仍为 true。 **/
    sent: boolean
}
[inline-code-end]

#### HTTP 方法

You can configure the HTTP method for each webhook event type in the admin panel:

- **Create 事件**：POST 或 PUT（默认：PUT）
- **Update 事件**：POST 或 PUT（默认：PUT）
- **Delete 事件**：DELETE、POST 或 PUT（默认：DELETE）

Since all requests contain an ID, Create and Update operations are idempotent by default (PUT). Repeating the same Create or Update request should not create duplicate objects on your side.

#### 请求头

Each webhook request includes the following headers:

| 头部 | 描述 |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | 您的 API 密钥 |
| `X-FastComments-Timestamp` | 请求签名时的 Unix 时间戳（秒） |
| `X-FastComments-Signature` | HMAC-SHA256 签名（`sha256=<hex>`） |

See [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.