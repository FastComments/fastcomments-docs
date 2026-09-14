A `FeedPost` 对象表示 FastComments 动态中的一篇帖子。动态是一系列带有各自评论线程的帖子流，由 Feed 小部件渲染。每篇帖子都有作者、可选的富内容、媒体和链接，并且可以被标记，以便对动态进行过滤。

`FeedPost` 对象的结构如下：

[inline-code-attrs-start title = 'FeedPost 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** 发布帖子的 FastComments 或 SSO 用户的 ID。 **/
    fromUserId?: string
    /** 未设置时从用户填充。 **/
    fromUserDisplayName?: string | null
    /** 只读。来自用户填充。 **/
    fromUserAvatar?: string | null
    /** 用于过滤动态。 **/
    tags?: string[]
    /** 在动态中的排序权重。值越高越靠前。 **/
    weight?: number
    /** 供您自行使用的自由键/值对。 **/
    meta?: Record<string, string>
    /** 已清理的 HTML。 **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY. Reaction type to count. **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** 点击时媒体项链接的目标地址。 **/
    linkUrl?: string
    /** 每个呈现对应一个条目。小部件会选择最合适的。 **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** 链接文本，例如 “立即注册”。 **/
    text?: string
    /** 链接显示的标题。 **/
    title?: string
    /** 链接显示的描述。 **/
    description?: string
    url?: string
}
[inline-code-end]

注意：

- 某些字段标记为 `READONLY` —— 这些字段由 API 返回，但不能被设置。
- 帖子上的评论是普通评论，其 `urlId` 为 `post:` 加上帖子 `_id`。使用该值配合评论 API 可读取或创建帖子的评论。