[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

此路由會建立單一個 `FeedPost`。每篇貼文都有作者，因此 `fromUserId` 為必填，且必須是帳號中已存在的 FastComments 或 SSO 使用者的 ID。

[inline-code-attrs-start title = 'FeedPost 建立 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 建立 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** 將貼文推送到目前在瀏覽器中開啟的資訊流。預設為 false。 **/
    isLive?: boolean
    /** 在儲存之前將貼文送入垃圾訊息引擎。預設為 false。 **/
    doSpamCheck?: boolean
    /** 跳過作為 doSpamCheck 一部分的重複內容檢查。預設為 false。 **/
    skipDupCheck?: boolean
    /** 最多 256 個字元。回傳給即時聽眾，使客戶端可以忽略自己的廣播。 **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** 必填。FastComments 或 SSO 使用者 ID。 **/
    fromUserId: string
    title?: string
    /** HTML。儲存時會進行清理。 **/
    contentHTML?: string
    /** 覆寫從使用者取得的顯示名稱。 **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 建立 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** 失敗時包含此欄位。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** 失敗時包含此欄位。 **/
    reason?: string
    feedPost?: FeedPost; // 成功時我們會回傳完整的已建立貼文。
}
[inline-code-end]