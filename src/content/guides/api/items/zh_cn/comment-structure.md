`Comment` 对象表示用户留下的评论。

父评论与子评论之间的关系通过 `parentId` 定义。

Comment 对象的结构如下：

[inline-code-attrs-start title = 'Comment 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: 如果垃圾评论引擎判定该评论为垃圾评论，则设置为 true。 **/
    aiDeterminedSpam?: boolean
    /** 该评论是否被批准显示。保存评论时设置为 true，否则将被隐藏。 **/
    approved?: boolean
    /** 用户的头像。 **/
    avatarSrc?: string
    /** 子评论。并非在所有场景下都会填充。当通过 API 将 asTree 设置为 true 时使用。 **/
    children: Comment[]
    /** 评论者的原始评论内容。 **/
    comment: string
    /** READONLY: 评论者的评论解析后的 HTML。 **/
    commentHTML?: string
    /** 评论者的邮箱。如果匿名评论功能关闭，则为必填项。 **/
    commenterEmail?: string
    /** 评论者的链接（例如，其博客）。 **/
    commenterLink?: string
    /** 评论者的名称。始终必填。如果不可用，请设置为类似 "Anonymous" 的值。 **/
    commenterName: string
    /** 评论发表的日期，以 UTC epoch 表示。 **/
    date: number
    /** 该评论的“显示标签”——例如 "Admin"、"Moderator"，或类似 "VIP User" 的内容。 **/
    displayLabel?: string
    /** 发表评论的域名。 **/
    domain?: string
    /** READONLY: 该评论被举报的次数。 **/
    flagCount?: number
    /** 评论中成功解析的 #hashtags 标签。你也可以手动添加 hashtags 用于查询，但它们不会自动显示在评论文本中。 **/
    hashTags?: CommentHashTag[]
    /** READONLY: 该评论是否包含图片？ **/
    hasImages?: boolean
    /** READONLY: 该评论是否包含链接？ **/
    hasLinks?: boolean
    /** READONLY: 唯一的评论 id。 **/
    id: string
    /** 仅在创建时有效！存储时会被哈希处理。 **/
    ip?: string
    /** READONLY: 当前用户是否屏蔽了撰写此评论的用户？ **/
    isBlocked?: boolean
    /** READONLY: 该评论是否由管理员发表？根据 userId 自动设置。 **/
    isByAdmin?: boolean
    /** READONLY: 该评论是否由版主发表？根据 userId 自动设置。 **/
    isByModerator?: boolean
    /** 如果该评论被软删除（由于其他配置而必须保留占位符），则设置为 true。 **/
    isDeleted?: boolean
    /** 如果用户的账户被删除而评论必须保留，则设置为 true。 **/
    isDeletedUser?: boolean
    /** READONLY: 是否被当前登录用户（contextUserId）举报？ **/
    isFlagged?: boolean
    /** 该评论是否被置顶？ **/
    isPinned?: boolean
    /** 该评论是否被锁定？为 true 时，任何人（包括版主）都无法回复、编辑或删除该评论，直到其被解锁。 **/
    isLocked?: boolean
    /** 该评论是否为垃圾评论？ **/
    isSpam?: boolean
    /** READONLY: 对于当前用户（contextUserId），该评论是否被投了反对票？ **/
    isVotedDown?: boolean
    /** READONLY: 对于当前用户（contextUserId），该评论是否被投了赞成票？ **/
    isVotedUp?: boolean
    /** 评论所在的语言区域。如果未提供，将从 HTTP 的 language accept 头部推导。 **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: 评论中成功解析的 @mentions。 **/
    mentions?: CommentUserMention[]
    /** 与该评论关联的可选元数据。 **/
    meta?: Record<string, string | number | boolean>
    /** 与该评论关联的审核组 id 的可选列表。 **/
    moderationGroupIds?: string[]|null
    /** READONLY: 对应于当前用户（contextUserId）对该评论投票的投票对象 id。 **/
    myVoteId?: string
    /** 是否已为该评论向评论者发送通知。为防止导入时发送通知，请将其设置为 true。 **/
    notificationSentForParent?: boolean
    /** 是否已为该评论向租户用户发送通知。为防止导入时发送通知，请将其设置为 true。 **/
    notificationSentForParentTenant?: boolean
    /** 该评论所在页面的标题。 **/
    pageTitle?: string
    /** 如果我们正在回复某条评论，这就是我们正在回复的评论 ID。 **/
    parentId?: string|null
    /** 该评论是否被标记为已审核。 **/
    reviewed: boolean
    /** 该评论所属的租户 id。 **/
    tenantId: string
    /** 撰写该评论的用户。保存带有名称/邮箱的评论时自动创建。 **/
    userId?: string|null
    /** 该评论可见的位置的 URL，例如博客文章。 **/
    url: string
    /** 你传给我们的 urlId 的“清理”版本。保存时你指定此字段，但当你取回该评论时，它会被“清理”，而你的原始值会被移到 "urlIdRaw"。 **/
    urlId: string
    /** READONLY: 你传给我们的原始 urlId。 **/
    urlIdRaw?: string
    /** 该用户和该评论是否已验证？ **/
    verified: boolean
    /** 赞成票数。 **/
    votesUp?: number
    /** 反对票数。 **/
    votesDown?: number
    /** 该评论的“karma”（= 赞成票数 - 反对票数）。 **/
    votes?: number
}
[inline-code-end]

其中一些字段被标记为 `READONLY`——这些字段由 API 返回，但无法设置。

### 评论文本结构

评论以 FastComments 风格的 markdown 编写，即传统 markdown 加上用于图片的传统 `bbcode` 风格标签，例如 `[img]path[/img]`。

文本存储在两个字段中。用户输入的文本未经修改地存储在 `comment` 字段中。它会被渲染并存储在 `commentHTML` 字段中。

允许的 HTML 标签为 `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`。

建议渲染 HTML，因为它只是 HTML 的一个很小的子集，构建一个渲染器非常简单。例如，React Native 和 Flutter 有多个库可以帮助实现这一点。

你可以选择渲染 `comment` 字段的未规范化值。[示例解析器在此。](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js)。

该示例解析器也可以调整为处理 HTML，将 HTML 标签转换为适合你的平台渲染的元素。

### 标注用户

当用户在评论中被标注时，信息会存储在一个名为 `mentions` 的列表中。该列表中的每个对象
具有以下结构。

[inline-code-attrs-start title = 'Comment Mentions 对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 用户 id。对于 SSO 用户，这将带有你的租户 id 前缀。 **/
    id: string
    /** 最终的 @mention 标签文本，包括 @ 符号。 **/
    tag: string
    /** 原始的 @mention 标签文本，包括 @ 符号。 **/
    rawTag: string
    /** 被标注的用户类型。user = FastComments.com 账户。sso = SSOUser。 **/
    type: 'user'|'sso'
    /** 如果用户选择不接收通知，此项仍会被设置为 true。 **/
    sent: boolean
}
[inline-code-end]

### HashTags

当使用 hashtags 并成功解析时，信息会存储在一个名为 `hashTags` 的列表中。该列表中的每个对象
具有以下结构。如果设置了 `retain`，hashtags 也可以手动添加到评论的 `hashTags` 数组中用于查询。

[inline-code-attrs-start title = 'Comment HashTag 对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** 该 hashtag 的 id。 **/
    id: string
    /** 最终的 #hashtag 标签文本，包括 # 符号。 **/
    tag: string
    /** 如果该 hashtag 关联了自定义 URL，则会定义此项。 **/
    url?: string
    /** 如果我们应在评论更新时保留该 hashtag（即使它不存在于评论文本中）。这对于在不更改评论文本的情况下标注评论很有用。 **/
    retain?: boolean
}
[inline-code-end]