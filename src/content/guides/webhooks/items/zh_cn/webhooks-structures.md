通过 webhook 发送的唯一结构是下面用 TypeScript 描述的 WebhookComment 对象。

#### WebhookComment 对象结构

##### “create” 事件结构
“create” 事件的请求正文是一个 WebhookComment 对象。

##### “update” 事件结构
“update” 事件的请求正文是一个 WebhookComment 对象。

##### “delete” 事件结构
“delete” 事件的请求正文是一个 WebhookComment 对象。

    变更（自 2023 年 11 月 14 日）
    此前，“delete” 事件的请求正文仅包含评论 id。现在它包含删除时的完整评论。


[inline-code-attrs-start title = 'WebhookComment 对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 评论的 id。 **/
    id: string
    /** 标识评论线程的 id 或 URL。已规范化。 **/
    urlId: string
    /** 指向评论所在位置的 URL。 **/
    url?: string
    /** 留下评论的用户 id。若为 SSO，则前缀包含租户 id。 **/
    userId?: string
    /** 留言用户的邮箱。 **/
    commenterEmail?: string
    /** 在评论组件中显示的用户名。对于 SSO，可能为 displayName。 **/
    commenterName: string
    /** 原始评论文本。 **/
    comment: string
    /** 解析后的评论文本。 **/
    commentHTML: string
    /** 评论的外部 id。 **/
    externalId?: string
    /** 父评论的 id。 **/
    parentId?: string | null
    /** 留言的 UTC 日期。 **/
    date: UTC_ISO_DateString
    /** 投票的综合评分（赞 - 踩）。 **/
    votes: number
    votesUp: number
    votesDown: number
    /** 如果用户发表评论时已登录，或他们已验证该评论，或在发表评论时验证了会话，则为 true。 **/
    verified: boolean
    /** 评论被验证的日期。 **/
    verifiedDate?: number
    /** 若版主将评论标记为已审核。 **/
    reviewed: boolean
    /** 头像的位置或 base64 编码。仅当 SSO 传入该值时才会是 base64。 **/
    avatarSrc?: string
    /** 评论是否被手动或自动标记为垃圾评论？ **/
    isSpam: boolean
    /** 评论是否被自动判定为垃圾评论？ **/
    aiDeterminedSpam: boolean
    /** 评论中是否包含图片？ **/
    hasImages: boolean
    /** 在“最相关”排序方向下，评论所在的页码。 **/
    pageNumber: number
    /** 在“最旧优先”排序方向下，评论所在的页码。 **/
    pageNumberOF: number
    /** 在“最新优先”排序方向下，评论所在的页码。 **/
    pageNumberNF: number
    /** 评论是自动还是手动批准的？ **/
    approved: boolean
    /** 评论撰写时用户的区域设置代码（格式：en_us）。 **/
    locale: string
    /** 评论中成功解析的 @提及 列表。 **/
    mentions?: CommentUserMention[]
    /** 评论来源的域名。 **/
    domain?: string
    /** 与此评论关联的可选审核组 id 列表。 **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

当用户在评论中被提及时，信息会存储在名为 `mentions` 的列表中。该列表中的每个对象具有以下结构。

[inline-code-attrs-start title = 'Webhook 提及对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 用户 id。对于 SSO 用户，将带有你的租户 id 前缀。 **/
    id: string
    /** 最终的 @提及 标签文本，包含 @ 符号。 **/
    tag: string
    /** 原始的 @提及 标签文本，包含 @ 符号。 **/
    rawTag: string
    /** 被提及的用户类型。user = FastComments.com 账户。sso = SSO 用户。 **/
    type: 'user'|'sso'
    /** 即使用户选择退出通知，该字段仍将设置为 true。 **/
    sent: boolean
}
[inline-code-end]

#### HTTP 方法

您可以在管理面板中为每种 webhook 事件类型配置 HTTP 方法：

- **Create 事件**：POST 或 PUT（默认：PUT）
- **Update 事件**：POST 或 PUT（默认：PUT）
- **Delete 事件**：DELETE、POST 或 PUT（默认：DELETE）

由于所有请求都包含 ID，Create 和 Update 操作默认是幂等的（PUT）。重复相同的 Create 或 Update 请求不应在你端创建重复对象。

#### 请求头

每个 webhook 请求都包含以下头：

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | 你的 API Secret |
| `X-FastComments-Timestamp` | 请求签名时的 Unix 时间戳（秒） |
| `X-FastComments-Signature` | HMAC-SHA256 签名（`sha256=<hex>`） |

请参阅 [安全与 API 令牌](/guides/webhooks/webhooks-api-tokens) 以获取有关验证 HMAC 签名的信息。

---