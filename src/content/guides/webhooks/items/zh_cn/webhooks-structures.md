The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### WebhookComment 对象结构

##### “Create” 事件结构  
The "create" event request body is a WebhookComment object.

##### “Update” 事件结构  
The "update" event request body is a WebhookComment object.

##### “Delete” 事件结构  
The "delete" event request body is a WebhookComment object.

Change as of Nov 14th 2023  
Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.

变更于 2023 年 11 月 14 日  
之前 “delete” 事件的请求体仅包含评论 ID。现在它在删除时包含完整的评论。

Every key is always present in the body. When the comment has no value for a field the body carries `null`  
(or `false` for booleans and `[]` for lists), so the shape of a delivery never varies from one comment to the next.

请求体中的每个键始终存在。当评论某字段没有值时，体中会携带 `null`（布尔值为 `false`，列表为 `[]`），因此每次交付的结构始终保持一致。

[inline-code-attrs-start title = 'WebhookComment 对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 评论的 ID。 **/
    id: string
    /** 标识评论线程的 ID 或 URL。已标准化。 **/
    urlId: string
    /** 指向评论所在位置的 URL。 **/
    url: string | null
    /** 留下评论的用户 ID。如果是 SSO，则前缀为租户 ID。 **/
    userId: string | null
    /** 发表评论用户的电子邮件。 **/
    commenterEmail: string | null
    /** 在评论小部件中显示的用户名称。使用 SSO 时，可为 displayName。 **/
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
    /** 评论验证的 UTC 日期。 **/
    verifiedDate: UTC_ISO_DateString | null
    /** 如果版主标记了评论已审阅。 **/
    reviewed: boolean
    /** 头像的位置或 base64 编码。如果是 SSO 传入的值，则仅为 base64。 **/
    avatarSrc: string | null
    /** 评论是否手动或自动标记为垃圾信息？ **/
    isSpam: boolean
    /** 评论是否自动标记为垃圾信息？ **/
    aiDeterminedSpam: boolean
    /** 评论中是否包含图片？ **/
    hasImages: boolean
    /** 评论在 “最相关” 排序方向上的页码。 **/
    pageNumber: number | null
    /** 评论在 “最旧优先” 排序方向上的页码。 **/
    pageNumberOF: number | null
    /** 评论在 “最新优先” 排序方向上的页码。 **/
    pageNumberNF: number | null
    /** 评论是自动还是手动批准的？ **/
    approved: boolean
    /** 用户撰写评论时的语言代码（格式：en_us）。 **/
    locale: string | null
    /** 评论中成功解析的 @mentions。若无则为空。 **/
    mentions: CommentUserMention[]
    /** 评论所属的域名。 **/
    domain: string | null
    /** 与此评论关联的审核组 ID。若无则为空。 **/
    moderationGroupIds: string[]
    /** 在此评论创建时显示在评论者名称旁的徽章。若无则为空。 **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

当用户在评论中被标记时，信息会存储在名为 `mentions` 的列表中。该列表中的每个对象具有以下结构。

[inline-code-attrs-start title = 'Webhook 提及对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 用户 ID。对于 SSO 用户，将在前面加上租户 ID。 **/
    id: string
    /** 最终的 @mention 标签文本，包括 @ 符号。 **/
    tag: string
    /** 原始的 @mention 标签文本，包括 @ 符号。 **/
    rawTag: string
    /** 被标记的用户类型。user = FastComments.com 账户。sso = SSOUser。 **/
    type: 'user'|'sso'
    /** 即使用户选择不接收通知，此字段仍会设为 true。 **/
    sent: boolean
}
[inline-code-end]

Each entry in the `badges` list has the following structure. See the [Badges guide](/guide-badges.html) for the badge types.

`badges` 列表中的每个条目具有以下结构。徽章类型请参见 [徽章指南](/guide-badges.html)。

[inline-code-attrs-start title = 'Webhook 徽章对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** 授予此用户的徽章 ID。 **/
    id: string
    /** 徽章类型。 **/
    type: number
    /** 徽章描述，悬停时显示。 **/
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

#### HTTP 方法

- **创建事件**：POST 或 PUT（默认：PUT）
- **更新事件**：POST 或 PUT（默认：PUT）
- **删除事件**：DELETE、POST 或 PUT（默认：DELETE）

由于所有请求都包含 ID，创建和更新操作默认是幂等的（PUT）。重复相同的创建或更新请求不应在您这边产生重复对象。

#### 请求头

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | 您的 API 密钥 |
| `X-FastComments-Timestamp` | 请求签名时的 Unix 时间戳（秒） |
| `X-FastComments-Signature` | HMAC-SHA256 签名（`sha256=<hex>`） |

See [Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.

有关验证 HMAC 签名的信息，请参见 [安全与 API 令牌](/guide-webhooks.html#webhooks-api-tokens)。

---