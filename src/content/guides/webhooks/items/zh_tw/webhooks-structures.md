透過 webhook 傳送的唯一結構是 WebhookComment 物件，以下以 TypeScript 說明。

#### WebhookComment 物件結構

##### 「建立」事件結構
「create」事件的請求主體是一個 WebhookComment 物件。

##### 「更新」事件結構
「update」事件的請求主體是一個 WebhookComment 物件。

##### 「刪除」事件結構
「delete」事件的請求主體是一個 WebhookComment 物件。

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'WebhookComment 物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 留言的 id。 **/
    id: string
    /** 識別留言串的 id 或 URL。已標準化。 **/
    urlId: string
    /** 指向留言位置的 URL。 **/
    url?: string
    /** 發表留言的使用者 id。若為 SSO，會加上租戶 id 前綴。 **/
    userId?: string
    /** 發表留言的使用者電子郵件。 **/
    commenterEmail?: string
    /** 在留言元件顯示的使用者名稱。SSO 情況下可能為 displayName。 **/
    commenterName: string
    /** 原始留言文字。 **/
    comment: string
    /** 經解析後的留言文字。 **/
    commentHTML: string
    /** 留言的外部 id。 **/
    externalId?: string
    /** 父留言的 id。 **/
    parentId?: string | null
    /** 留言發表的 UTC 日期。 **/
    date: UTC_ISO_DateString
    /** 投票的綜合評分 (贊成 - 反對)。 **/
    votes: number
    votesUp: number
    votesDown: number
    /** 若使用者留言時已登入、或其評論已被驗證、或在留言時已驗證其會話，則為 true。 **/
    verified: boolean
    /** 留言被驗證的日期。 **/
    verifiedDate?: number
    /** 是否由管理員標記為已審閱。 **/
    reviewed: boolean
    /** 大頭照的位置或 base64 編碼。僅在透過 SSO 傳遞 base64 時才會是 base64。 **/
    avatarSrc?: string
    /** 留言是否被手動或自動標記為垃圾訊息？ **/
    isSpam: boolean
    /** 留言是否被自動判定為垃圾訊息？ **/
    aiDeterminedSpam: boolean
    /** 留言中是否包含圖片？ **/
    hasImages: boolean
    /** 在「最相關」排序方向下，留言所在的頁碼。 **/
    pageNumber: number
    /** 在「最舊優先」排序方向下，留言所在的頁碼。 **/
    pageNumberOF: number
    /** 在「最新優先」排序方向下，留言所在的頁碼。 **/
    pageNumberNF: number
    /** 留言是否被自動或手動核准？ **/
    approved: boolean
    /** 使用者撰寫留言時的地區代碼 (格式: en_us)。 **/
    locale: string
    /** 留言中成功解析出的 @提及。 **/
    mentions?: CommentUserMention[]
    /** 留言所屬的網域。 **/
    domain?: string
    /** 與此留言相關的可選審核群組 id 列表。 **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Webhook 提及物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 使用者 id。對於 SSO 使用者，會加上租戶 id 前綴。 **/
    id: string
    /** 最終的 @提及 標籤文字，包含 @ 符號。 **/
    tag: string
    /** 原始的 @提及 標籤文字，包含 @ 符號。 **/
    rawTag: string
    /** 被標註的使用者類型。user = FastComments.com 帳號。sso = SSOUser. **/
    type: 'user'|'sso'
    /** 即使使用者選擇不接收通知，此欄位仍會設為 true。 **/
    sent: boolean
}
[inline-code-end]

#### HTTP 方法

您可以在管理面板中為每種 webhook 事件類型設定 HTTP 方法：

- **Create Event**: POST or PUT (default: PUT)
- **Update Event**: POST or PUT (default: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE)

由於所有請求都包含 ID，Create 與 Update 操作預設為冪等 (PUT)。重複相同的 Create 或 Update 請求不應該在您的端造成重複的物件。

#### 請求標頭

每個 webhook 請求會包含以下標頭：

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Your API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Signature` | HMAC-SHA256 signature (`sha256=<hex>`) |

See [安全性與 API 令牌](/guide-webhooks.html#webhooks-api-tokens) for information on verifying the HMAC signature.