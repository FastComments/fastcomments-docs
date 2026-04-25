A `Comment` object represents a comment left by a user.

The relationship between parent and child comments is defined via `parentId`.

The structure for the Comment object is as follows:

[inline-code-attrs-start title = 'Comment 对象结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** 只读：如果反垃圾引擎判定该评论为垃圾评论则设为 true。 **/
    aiDeterminedSpam?: boolean
    /** 评论是否已批准显示。保存评论时设置为 true，否则将被隐藏。 **/
    approved?: boolean
    /** 用户的头像。 **/
    avatarSrc?: string
    /** 子评论。在并非所有场景中都会填充。用于当通过 API 将 asTree 设置为 true 时。 **/
    children: Comment[]
    /** 评论者的原始评论。 **/
    comment: string
    /** 只读：评论者的评论解析后的 HTML。 **/
    commentHTML?: string
    /** 评论者的邮箱。如果匿名评论被禁用则必填。 **/
    commenterEmail?: string
    /** 评论者的链接（例如，他们的博客）。 **/
    commenterLink?: string
    /** 评论者的姓名。始终必填。如果不可用，请设置为类似 "Anonymous" 的值。 **/
    commenterName: string
    /** 留言的日期，UTC 时间戳。 **/
    date: number
    /** 评论的“显示标签”，例如 "Admin", "Moderator", 或类似 "VIP User" 的标签。 **/
    displayLabel?: string
    /** 发布评论的域名。 **/
    domain?: string
    /** 只读：该评论被标记的次数。 **/
    flagCount?: number
    /** 评论中成功解析的 #hashtags。您也可以手动添加 hashtag 以便查询，但它们不会自动显示在评论文本中。 **/
    hashTags?: CommentHashTag[]
    /** 只读：评论是否包含图片？ **/
    hasImages?: boolean
    /** 只读：评论是否包含链接？ **/
    hasLinks?: boolean
    /** 只读：唯一的评论 id。 **/
    id: string
    /** 仅在创建时使用！该值会被哈希后存储。 **/
    ip?: string
    /** 只读：当前用户是否屏蔽了写此评论的用户？ **/
    isBlocked?: boolean
    /** 只读：该评论是否由管理员发布？基于 userId 自动设置。 **/
    isByAdmin?: boolean
    /** 只读：该评论是否由版主发布？基于 userId 自动设置。 **/
    isByModerator?: boolean
    /** 如果该评论被软删除（由于其他配置必须保留占位符），则设为 true。 **/
    isDeleted?: boolean
    /** 如果用户帐户被删除且评论必须保留，则设为 true。 **/
    isDeletedUser?: boolean
    /** 只读：当前登录用户（contextUserId）是否已标记该评论？ **/
    isFlagged?: boolean
    /** 评论是否被置顶？ **/
    isPinned?: boolean
    /** 评论是否被锁定？为 true 时，任何人（包括版主）在解锁之前都不能回复、编辑或删除该评论。 **/
    isLocked?: boolean
    /** 该评论是否为垃圾评论？ **/
    isSpam?: boolean
    /** 只读：对当前用户（contextUserId）而言该评论是否被点踩？ **/
    isVotedDown?: boolean
    /** 只读：对当前用户（contextUserId）而言该评论是否被点赞？ **/
    isVotedUp?: boolean
    /** 评论的 locale。如果未提供，将从 HTTP 的 Accept-Language 头推断。 **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** 只读：评论中成功解析的 @mentions。 **/
    mentions?: CommentUserMention[]
    /** 与评论关联的可选元数据。 **/
    meta?: Record<string, string | number | boolean>
    /** 与该评论关联的可选审核组 ID 列表。 **/
    moderationGroupIds?: string[]|null
    /** 只读：对应当前用户（contextUserId）对该评论投票的投票对象 ID。 **/
    myVoteId?: string
    /** 是否已向评论者发送了关于此评论的通知。若要在导入时阻止发送通知，请将此设置为 true。 **/
    notificationSentForParent?: boolean
    /** 是否已向租户用户发送了关于此评论的通知。若要在导入时阻止发送通知，请将此设置为 true。 **/
    notificationSentForParentTenant?: boolean
    /** 该评论所在页面的标题。 **/
    pageTitle?: string
    /** 如果我们正在回复某条评论，则此字段为被回复的评论 ID。 **/
    parentId?: string|null
    /** 评论是否被标记为已审核。 **/
    reviewed: boolean
    /** 评论所属的租户 ID。 **/
    tenantId: string
    /** 撰写该评论的用户。使用姓名/邮箱保存评论时会自动创建。 **/
    userId?: string|null
    /** 该评论可见位置的 URL，例如博客文章。 **/
    url: string
    /** 您提供的 urlId 的“清理”版本。保存时您指定此字段，但在获取评论时，该字段将被“清理”，原始值会移动到 "urlIdRaw"。 **/
    urlId: string
    /** 只读：您传入的原始 urlId。 **/
    urlIdRaw?: string
    /** 用户和该评论是否已验证？ **/
    verified: boolean
    /** 点赞数。 **/
    votesUp?: number
    /** 点踩数。 **/
    votesDown?: number
    /** 评论的“karma”（= 点赞数 - 点踩数）。 **/
    votes?: number
}
[inline-code-end]

Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.

### 评论文本结构

Comments are written in a FastComments flavor of markdown, which is just markdown plus traditional `bbcode` style tags for images, like `[img]path[/img]`.

Text is stored in two fields. The text the user entered is stored unmodified in the `comment` field. This is rendered and stored in the `commentHTML` field.

The allowed HTML tags are `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`.

It's recommended to render the HTML, since it is a very small subset of HTML, building a renderer is pretty straightforward. There are multiple libraries for React Native and Flutter, for instance, to help with this

You may choose to render the un-normalized value of the `comment` field. [An example parser is here.](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js).

The example parser could also be adjusted to work with HTML, and transform the HTML tags into expected elements to render for your platform. 

### 标记

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = '评论提及对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 用户 ID。对于 SSO 用户，会以您的租户 ID 作为前缀。 **/
    id: string
    /** 最终的 @mention 标签文本，包括 @ 符号。 **/
    tag: string
    /** 原始的 @mention 标签文本，包括 @ 符号。 **/
    rawTag: string
    /** 被标记的用户类型。user = FastComments.com 账户。sso = SSOUser。 **/
    type: 'user'|'sso'
    /** 即使用户选择退出通知，该字段仍会被设置为 true。 **/
    sent: boolean
}
[inline-code-end]

### 话题标签（HashTags）

When hashtags are used and successfully parsed, the information is stored in a list called `hashTags`. Each object in that list
has the following structure. Hashtags can also be manually added to the comment `hashTags` array for querying, if `retain` is set.

[inline-code-attrs-start title = '评论 HashTag 对象'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** hashtag 的 ID. **/
    id: string
    /** 最终的 #hashtag 标签文本，包括 # 符号。 **/
    tag: string
    /** 如果该 hashtag 关联了自定义 URL，则该字段会被定义。 **/
    url?: string
    /** 当评论被更新时，是否应保留该 hashtag，即使它不出现在评论文本中也会保留。对于在不修改评论文本的情况下标记评论很有用。 **/
    retain?: boolean
}
[inline-code-end]

---