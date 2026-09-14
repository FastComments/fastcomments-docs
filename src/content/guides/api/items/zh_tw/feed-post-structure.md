A `FeedPost` 物件代表 FastComments 供稿中的一篇貼文。供稿是一系列貼文的串流，每篇貼文都有自己的評論串，透過 Feed 小工具呈現。每篇貼文都有作者、可選的富內容、媒體與連結，且可以加上標籤，以便過濾供稿。

`FeedPost` 物件的結構如下：

[inline-code-attrs-start title = 'FeedPost 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** 貼文作者的 FastComments 或 SSO 使用者 ID。 **/
    fromUserId?: string
    /** 未設定時由使用者填入。 **/
    fromUserDisplayName?: string | null
    /** READONLY。由使用者填入。 **/
    fromUserAvatar?: string | null
    /** 用於過濾供稿。 **/
    tags?: string[]
    /** 供稿內的排序權重。值越高越優先。 **/
    weight?: number
    /** 供自行使用的自由鍵/值對。 **/
    meta?: Record<string, string>
    /** 已清理的 HTML。 **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY。回應類型計數。 **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** 點擊時媒體項目的連結目標。 **/
    linkUrl?: string
    /** 每個不同尺寸各有一筆條目。小工具會挑選最適合的。 **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** 連結文字，例如「立即註冊」。 **/
    text?: string
    /** 連結旁顯示的標題。 **/
    title?: string
    /** 連結旁顯示的描述。 **/
    description?: string
    url?: string
}
[inline-code-end]

注意事項：

- 其中一些欄位標記為 `READONLY` —— 這些欄位由 API 回傳，但無法設定。
- 貼文的評論是一般評論，其 `urlId` 為 `post:` 加上貼文的 `_id`。使用此值搭配評論 API，即可讀取或建立貼文的評論。