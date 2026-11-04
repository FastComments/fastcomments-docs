通过 webhook 发送的唯一结构是 WebhookComment 对象，其 TypeScript 定义如下。

#### WebhookComment 对象结构

##### "Create" 事件结构
"create" 事件的请求体是一个 WebhookComment 对象。

##### "Update" 事件结构
"update" 事件的请求体是一个 WebhookComment 对象。

##### "Delete" 事件结构
"delete" 事件的请求体是一个 WebhookComment 对象。

    2023年11月14日的变更
    之前 "delete" 事件的请求体只包含评论 id。现在它包含删除时的完整评论。

请求体中始终包含每个键。当评论的某个字段没有值时，请求体中为 `null`
（布尔值为 `false`，列表为 `[]`），因此每次投递的数据结构始终一致。

[inline-code-attrs-start title = 'WebhookComment 对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 评论的 id。 **/
    id: string
    /** 标识评论线程的 id 或 URL。已规范化。 **/
    urlId: string
    /** 指向评论所在位置的 URL。 **/
    url: string | null
    /** 发表评论的用户 id。如果是 SSO，则带有租户 id 前缀。 **/
    userId: string | null
    /** 发表评论的用户的邮箱。 **/
    commenterEmail: string | null
    /** 在评论组件中显示的用户名。使用 SSO 时，可以是 displayName。 **/
    commenterName: string
    /** 原始评论文本。 **/
    comment: string
    /** 解析后的评论文本。 **/
    commentHTML: string
    /** 评论的外部 id。 **/
    externalId: string | null
    /** 父评论的 id。 **/
    parentId: string | null
    /** 发表评论时的 UTC 日期。 **/
    date: UTC_ISO_DateString
    /** 投票的总 karma（赞 - 踩）。 **/
    votes: number
    votesUp: number
    votesDown: number
    /** 如果用户评论时已登录、或已验证评论、或在评论时验证了会话，则为 true。 **/
    verified: boolean
    /** 评论被验证时的 UTC 日期。 **/
    verifiedDate: UTC_ISO_DateString | null
    /** 版主是否已将该评论标记为已审核。 **/
    reviewed: boolean
    /** 头像的位置或 base64 编码。仅当 SSO 传入的值是 base64 时才会是 base64。 **/
    avatarSrc: string | null
    /** 评论是被手动还是自动标记为垃圾评论？ **/
    isSpam: boolean
    /** 评论是否被自动标记为垃圾评论？ **/
    aiDeterminedSpam: boolean
    /** 评论中是否包含图片？ **/
    hasImages: boolean
    /** 按 "Most Relevant"（最相关）排序方向评论所在的页码。 **/
    pageNumber: number | null
    /** 按 "Oldest First"（最早优先）排序方向评论所在的页码。 **/
    pageNumberOF: number | null
    /** 按 "Newest First"（最新优先）排序方向评论所在的页码。 **/
    pageNumberNF: number | null
    /** 评论是被自动还是手动批准的？ **/
    approved: boolean
    /** 用户撰写评论时的区域设置代码（格式：en_us）。 **/
    locale: string | null
    /** 评论中成功解析的 @mentions。没有时为空。 **/
    mentions: CommentUserMention[]
    /** 评论所属的域名。 **/
    domain: string | null
    /** 与此评论关联的审核组 id。没有时为空。 **/
    moderationGroupIds: string[]
    /** 此评论中评论者名称旁显示的徽章，以事件创建时为准。没有时为空。 **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

当用户在评论中被 @ 提及时，相关信息存储在名为 `mentions` 的列表中。该列表中的每个对象
具有以下结构。

[inline-code-attrs-start title = 'Webhook Mentions 对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 用户 id。对于 SSO 用户，将带有您的租户 id 前缀。 **/
    id: string
    /** 最终的 @mention 标签文本，包括 @ 符号。 **/
    tag: string
    /** 原始的 @mention 标签文本，包括 @ 符号。 **/
    rawTag: string
    /** 被提及的用户类型。user = FastComments.com 账户。sso = SSOUser。 **/
    type: 'user'|'sso'
    /** 即使用户选择不接收通知，此项仍会设置为 true。 **/
    sent: boolean
}
[inline-code-end]

`badges` 列表中的每一项具有以下结构。徽章类型请参阅[徽章指南](/guide-badges.html)。

[inline-code-attrs-start title = 'Webhook 徽章对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** 授予该用户的徽章 id。 **/
    id: string
    /** 徽章类型。 **/
    type: number
    /** 徽章描述，悬停在徽章上时显示。 **/
    description: string
    /** 徽章上显示的文本。 **/
    displayLabel?: string | null
    /** 徽章使用的图片（如果使用图片）。 **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** 应用于徽章的额外 CSS 类。 **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP 方法

您可以在管理面板中为每种 webhook 事件类型配置 HTTP 方法：

- **Create 事件**：POST 或 PUT（默认：PUT）
- **Update 事件**：POST 或 PUT（默认：PUT）
- **Delete 事件**：DELETE、POST 或 PUT（默认：DELETE）

由于所有请求都包含 ID，Create 和 Update 操作默认（PUT）是幂等的。重复相同的 Create 或 Update 请求不应在您那边创建重复的对象。

#### 请求头

每个 webhook 请求包含以下请求头：

| 请求头 | 描述 |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | 您的 API Secret |
| `X-FastComments-Timestamp` | 请求签名时的 Unix 时间戳（秒） |
| `X-FastComments-Signature` | HMAC-SHA256 签名（`sha256=<hex>`） |

有关验证 HMAC 签名的信息，请参阅[安全与 API 令牌](/guide-webhooks.html#webhooks-api-tokens)。