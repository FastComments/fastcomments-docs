通过 webhooks 发送的唯一结构是下面用 TypeScript 描述的 WebhookComment 对象。

#### WebhookComment 对象结构

##### "Create" 事件结构
"create" 事件的请求正文是一个 WebhookComment 对象。

##### "Update" 事件结构
"update" 事件的请求正文是一个 WebhookComment 对象。

##### "Delete" 事件结构
"delete" 事件的请求正文是一个 WebhookComment 对象。

    自 2023 年 11 月 14 日 的更改
    之前，"delete" 事件的请求正文仅包含评论 id。现在它包含删除时的完整评论。


[inline-code-attrs-start title = 'WebhookComment 对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 评论的 id。 **/
    id: string
    /** 标识评论线程的 id 或 URL。已规范化。 **/
    urlId: string
    /** 指向评论所在位置的 URL。 **/
    url?: string
    /** 留下评论的用户 id。如果是 SSO，会加上租户 id 前缀。 **/
    userId?: string
    /** 留下评论的用户的电子邮件。 **/
    commenterEmail?: string
    /** 在评论组件中显示的用户名。使用 SSO 时，可能是 displayName。 **/
    commenterName: string
    /** 原始评论文本。 **/
    comment: string
    /** 解析后的评论文本。 **/
    commentHTML: string
    /** 评论的外部 id。 **/
    externalId?: string
    /** 父评论的 id。 **/
    parentId?: string | null
    /** 评论创建时的 UTC 日期。 **/
    date: UTC_ISO_DateString
    /** 投票的综合评分（赞 - 踩）。 **/
    votes: number
    votesUp: number
    votesDown: number
    /** 如果用户发表评论时已登录、或他们验证了该评论、或他们在发表评论时验证了会话，则为 true。 **/
    verified: boolean
    /** 评论验证的日期。 **/
    verifiedDate?: number
    /** 是否有版主将评论标记为已审核。 **/
    reviewed: boolean
    /** 头像的位置或 base64 编码。仅当通过 SSO 传入该值时才会是 base64。 **/
    avatarSrc?: string
    /** 评论是否被手动或自动标记为垃圾信息？ **/
    isSpam: boolean
    /** 评论是否被自动判定为垃圾信息？ **/
    aiDeterminedSpam: boolean
    /** 评论中是否包含图片？ **/
    hasImages: boolean
    /** 在“最相关”排序方向下，评论所在的页码。 **/
    pageNumber: number
    /** 在“最旧优先”排序方向下，评论所在的页码。 **/
    pageNumberOF: number
    /** 在“最新优先”排序方向下，评论所在的页码。 **/
    pageNumberNF: number
    /** 评论是自动批准还是手动批准的？ **/
    approved: boolean
    /** 评论撰写时用户的区域代码（格式：en_us）。 **/
    locale: string
    /** 评论中已成功解析的 @mentions。 **/
    mentions?: CommentUserMention[]
    /** 评论来源的域名。 **/
    domain?: string
    /** 与此评论关联的可选审核组 id 列表。 **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

当用户在评论中被标记时，信息会存储在名为 `mentions` 的列表中。该列表中的每个对象具有以下结构。

[inline-code-attrs-start title = 'Webhook 提及对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 用户 id。对于 SSO 用户，会带上租户 id 前缀。 **/
    id: string
    /** 最终的 @mention 标签文本，包括 @ 符号。 **/
    tag: string
    /** 原始的 @mention 标签文本，包括 @ 符号。 **/
    rawTag: string
    /** 被标记用户的类型。user = FastComments.com 账户。sso = SSOUser。 **/
    type: 'user'|'sso'
    /** 如果用户选择退出通知，该字段仍会被设置为 true。 **/
    sent: boolean
}
[inline-code-end]

#### 使用的 HTTP 方法

**Create 和 Update 都使用 HTTP PUT 而不是 POST！**

由于我们所有请求都包含 ID，重复相同的 Create 或 Update 请求不应该在您那边创建新的对象。

这意味着这些调用是幂等的，根据 HTTP 规范应使用 PUT 事件。

---