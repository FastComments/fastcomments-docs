The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### WebhookComment 物件結構

##### 「建立」事件結構
「create」事件的請求主體是一個 WebhookComment 物件。

##### 「更新」事件結構
「update」事件的請求主體是一個 WebhookComment 物件。

##### 「刪除」事件結構
「delete」事件的請求主體是一個 WebhookComment 物件。

    變更於 2023 年 11 月 14 日
    之前「delete」事件的請求主體僅包含評論 ID。現在會在刪除時包含完整的評論內容。

每個鍵在主體中皆會出現。當評論的某個欄位沒有值時，主體會攜帶 `null`（布林值則為 `false`，列表則為 `[]`），因此每次傳遞的資料形狀不會因評論而異。

[inline-code-attrs-start title = 'WebhookComment 物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** The id of the comment. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** The URL that points to where the comment was left. **/
    url: string | null
    /** The user id that left the comment. If SSO, prefixed with tenant id. **/
    userId: string | null
    /** The email of the user left the comment. **/
    commenterEmail: string | null
    /** The name of the user that shows in the comment widget. With SSO, can be displayName. **/
    commenterName: string
    /** Raw comment text. **/
    comment: string
    /** Comment text after parsing. **/
    commentHTML: string
    /** Comment external id. **/
    externalId: string | null
    /** The id of the parent comment. **/
    parentId: string | null
    /** The UTC date when the comment was left. **/
    date: UTC_ISO_DateString
    /** Combined karma (up - down) of votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True if the user was logged in when they commented, or their verified the comment, or if they verified their session when the comment was left. **/
    verified: boolean
    /** The UTC date when the comment was verified. **/
    verifiedDate: UTC_ISO_DateString | null
    /** If a moderator marked the comment reviewed. **/
    reviewed: boolean
    /** The location, or base64 encoding, of the avatar. Will only be base64 if that was the value passed with SSO. **/
    avatarSrc: string | null
    /** Was the comment manually or automatically marked as spam? **/
    isSpam: boolean
    /** Was the comment automatically marked as spam? **/
    aiDeterminedSpam: boolean
    /** Are there images in the comment? **/
    hasImages: boolean
    /** The page number the comment is on for the "Most Relevant" sort direction. **/
    pageNumber: number | null
    /** The page number the comment is on for the "Oldest First" sort direction. **/
    pageNumberOF: number | null
    /** The page number the comment is on for the "Newest First" sort direction. **/
    pageNumberNF: number | null
    /** Was the comment approved automatically or manually? **/
    approved: boolean
    /** The locale code (format: en_us) of the user when the comment was written. **/
    locale: string | null
    /** The @mentions written in the comment that were successfully parsed. Empty when there are none. **/
    mentions: CommentUserMention[]
    /** The domain the comment is from. **/
    domain: string | null
    /** The moderation group ids associated with this comment. Empty when there are none. **/
    moderationGroupIds: string[]
    /** The badges shown next to the commenter's name on this comment, as of when the event was created. Empty when there are none. **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

當使用者在評論中被標記時，資訊會儲存在名為 `mentions` 的列表中。該列表中的每個物件具有以下結構。

[inline-code-attrs-start title = 'Webhook 提及 物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    sent: boolean
}
[inline-code-end]

`badges` 列表中的每個條目具有以下結構。請參閱 [徽章指南](/guide-badges.html) 了解徽章類型。

[inline-code-attrs-start title = 'Webhook 徽章 物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** The id of the badge awarded to this user. **/
    id: string
    /** The badge type. **/
    type: number
    /** The badge description, shown when hovering over the badge. **/
    description: string
    /** The text shown on the badge. **/
    displayLabel?: string | null
    /** The image shown for the badge, if it uses one. **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** An extra CSS class applied to the badge. **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP 方法

您可以在管理面板中為每種 webhook 事件類型設定 HTTP 方法：

- **建立事件**：POST 或 PUT（預設：PUT）
- **更新事件**：POST 或 PUT（預設：PUT）
- **刪除事件**：DELETE、POST 或 PUT（預設：DELETE）

由於所有請求皆包含 ID，建立與更新操作預設為冪等（PUT）。重複相同的建立或更新請求不應在您的端產生重複的物件。

#### 請求標頭

每個 webhook 請求都會包含以下標頭：

| 標頭 | 說明 |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | 您的 API 密鑰 |
| `X-FastComments-Timestamp` | 請求簽署時的 Unix 時間戳（秒） |
| `X-FastComments-Signature` | HMAC-SHA256 簽名（`sha256=<hex>`） |

請參閱 [安全性與 API 令牌](/guide-webhooks.html#webhooks-api-tokens) 以取得驗證 HMAC 簽名的相關資訊。