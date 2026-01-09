一个 `Comment` 对象表示用户留下的一条评论。

父评论与子评论之间的关系通过 `parentId` 定义。

Comment 对象的结构如下：

[inline-code-attrs-start title = '评论结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** 只读：如果反垃圾引擎判定该评论为垃圾，则设置为 true。 **/
    aiDeterminedSpam?: boolean
    /** 评论是否被批准显示。保存评论时设置为 true，否则将被隐藏。 **/
    approved?: boolean
    /** 用户的头像。 **/
    avatarSrc?: string
    /** 子评论。并非在所有场景中都会填充。用于通过 API 将 asTree 设置为 true 时。 **/
    children: Comment[]
    /** 评论者的原始评论。 **/
    comment: string
    /** 只读：将评论者的评论解析为 HTML。 **/
    commentHTML?: string
    /** 评论者的邮箱。如果关闭匿名评论，则为必填。 **/
    commenterEmail?: string
    /** 评论者的链接（例如，他们的博客）。 **/
    commenterLink?: string
    /** 评论者的姓名。始终必填。如果不可用，请设置为类似“匿名”的值。 **/
    commenterName: string
    /** 评论留存的日期，UTC 纪元时间。 **/
    date: number
    /** 评论的“显示标签”——例如 “Admin”、“Moderator” 或类似 “VIP User”。 **/
    displayLabel?: string
    /** 发布该评论的域名。 **/
    domain?: string
    /** 只读：该评论被举报的次数。 **/
    flagCount?: number
    /** 评论中成功解析的 #hashtags。你也可以手动添加标签以便查询，但它们不会自动在评论文本中显示。 **/
    hashTags?: CommentHashTag[]
    /** 只读：评论是否包含图片？ **/
    hasImages?: boolean
    /** 只读：评论是否包含链接？ **/
    hasLinks?: boolean
    /** 只读：唯一评论 ID。 **/
    id: string
    /** 仅在创建时使用！此项会被哈希后存储。 **/
    ip?: string
    /** 只读：当前用户是否屏蔽了撰写此评论的用户？ **/
    isBlocked?: boolean
    /** 只读：该评论是否由管理员发布？基于 userId 自动设置。 **/
    isByAdmin?: boolean
    /** 只读：该评论是否由版主发布？基于 userId 自动设置。 **/
    isByModerator?: boolean
    /** 如果评论被软删除（由于其他配置必须保留占位符），则设置为 true。 **/
    isDeleted?: boolean
    /** 如果用户账户被删除且必须保留该评论，则设置为 true。 **/
    isDeletedUser?: boolean
    /** 只读：当前登录用户（contextUserId）是否已举报该评论？ **/
    isFlagged?: boolean
    /** 该评论是否被置顶？ **/
    isPinned?: boolean
    /** 该评论是否锁定以禁止新回复（版主仍可回复）？ **/
    isLocked?: boolean
    /** 该评论是否为垃圾评论？ **/
    isSpam?: boolean
    /** 只读：当前用户（contextUserId）是否对该评论点踩？ **/
    isVotedDown?: boolean
    /** 只读：当前用户（contextUserId）是否对该评论点赞？ **/
    isVotedUp?: boolean
    /** 评论使用的语言环境。如果未提供，将从 HTTP 的 Accept-Language 头部推断。 **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** 只读：评论中成功解析的 @提及 列表。 **/
    mentions?: CommentUserMention[]
    /** 可选的与评论关联的元数据。 **/
    meta?: Record<string, string | number | boolean>
    /** 与此评论关联的可选版务组 ID 列表。 **/
    moderationGroupIds?: string[]|null
    /** 只读：对应当前用户（contextUserId）对该评论的投票对象的 ID。 **/
    myVoteId?: string
    /** 是否已向评论者发送针对该评论的通知。为防止在导入时发送通知，请将其设置为 true。 **/
    notificationSentForParent?: boolean
    /** 是否已向租户用户发送针对该评论的通知。为防止在导入时发送通知，请将其设置为 true。 **/
    notificationSentForParentTenant?: boolean
    /** 此评论所在页面的标题。 **/
    pageTitle?: string
    /** 如果我们在回复某条评论，则这是我们回复的评论的 ID。 **/
    parentId?: string|null
    /** 评论是否被标记为已审核。 **/
    reviewed: boolean
    /** 评论所属的租户 ID。 **/
    tenantId: string
    /** 撰写该评论的用户。当使用姓名/邮箱保存评论时会自动创建。 **/
    userId?: string|null
    /** 该评论可见位置的 URL，例如一篇博文的链接。 **/
    url: string
    /** 你提供的 urlId 的“清理”版本。保存时你会指定此字段，但当你取回评论时，该字段会被“清理”，你原始的值会移动到 urlIdRaw。 **/
    urlId: string
    /** 只读：你传递的原始 urlId。 **/
    urlIdRaw?: string
    /** 用户和该评论是否已验证？ **/
    verified: boolean
    /** 点赞数量。 **/
    votesUp?: number
    /** 点踩数量。 **/
    votesDown?: number
    /** 评论的“声望”（= 点赞数 - 点踩数）。 **/
    votes?: number
}
[inline-code-end]

其中一些字段标注为 `READONLY` —— 这些字段由 API 返回，但无法设置。

### 评论文本结构

评论采用 FastComments 风格的 markdown 编写，即在普通 markdown 基础上增加了传统的 `bbcode` 样式图片标签，例如 `[img]path[/img]`。

文本存储在两个字段中。用户输入的文本未作修改地保存在 `comment` 字段中。渲染后的结果则存储在 `commentHTML` 字段中。

允许的 HTML 标签有 `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`。

建议渲染该 HTML，因为它只是 HTML 的一个很小的子集，构建渲染器相对简单。例如，有多个用于 React Native 和 Flutter 的库可以提供帮助

你也可以选择渲染 `comment` 字段的未规范化值。[示例解析器在这里。](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js)。

示例解析器也可以调整以处理 HTML，并将 HTML 标签转换为适合你平台渲染的元素。 

### 提及

当用户在评论中被提及时，信息会存储在名为 `mentions` 的列表中。该列表中的每个对象具有以下结构。

[inline-code-attrs-start title = '评论提及对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 用户 ID。对于 SSO 用户，会在前面加上你的租户 ID 前缀。 **/
    id: string
    /** 最终的 @ 提及 标签文本，包括 @ 符号。 **/
    tag: string
    /** 原始的 @ 提及 标签文本，包括 @ 符号。 **/
    rawTag: string
    /** 被提及的用户类型。user = FastComments.com 帐户。sso = SSO 用户。 **/
    type: 'user'|'sso'
    /** 即使用户选择不接收通知，该字段仍会被设置为 true。 **/
    sent: boolean
}
[inline-code-end]

### 话题标签

当使用的 hashtag 被成功解析时，信息会存储在名为 `hashTags` 的列表中。该列表中的每个对象具有以下结构。如果设置了 `retain`，也可以将话题标签手动添加到评论的 `hashTags` 数组中以便查询。

[inline-code-attrs-start title = '评论话题标签对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** 话题标签 ID。 **/
    id: string
    /** 最终的 #hashtag 标签文本，包括 # 符号。 **/
    tag: string
    /** 如果该话题标签关联了自定义 URL，则会定义此字段。 **/
    url?: string
    /** 指在更新评论时是否应保留该话题标签，即使它不出现在评论文本中。用于在不更改评论文本的情况下标记评论。 **/
    retain?: boolean
}
[inline-code-end]

---