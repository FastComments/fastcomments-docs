透過 webhook 傳送的唯一結構為下方以 TypeScript 描述的 WebhookComment 物件。

#### WebhookComment 物件結構

##### "Create" 事件結構
"create" 事件的請求主體為 WebhookComment 物件。

##### "Update" 事件結構
"update" 事件的請求主體為 WebhookComment 物件。

##### "Delete" 事件結構
"delete" 事件的請求主體為 WebhookComment 物件。

    Change as of Nov 14th 2023
    Previously the "delete" event request body only contained the comment id. It now contains the full comment at the time of deletion.


[inline-code-attrs-start title = 'WebhookComment 物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 評論的 id。 **/
    id: string
    /** 用來識別評論串的 id 或 URL。已標準化。 **/
    urlId: string
    /** 指向留言位置的 URL。 **/
    url?: string
    /** 留下評論的使用者 id。若為 SSO，會加上租戶 id 的前綴。 **/
    userId?: string
    /** 留下評論的使用者電子郵件。 **/
    commenterEmail?: string
    /** 在評論元件中顯示的使用者名稱。若使用 SSO，可能為 displayName。 **/
    commenterName: string
    /** 原始評論文字。 **/
    comment: string
    /** 經解析後的評論文字。 **/
    commentHTML: string
    /** 評論的外部 id。 **/
    externalId?: string
    /** 父評論的 id。 **/
    parentId?: string | null
    /** 留下評論的 UTC 日期。 **/
    date: UTC_ISO_DateString
    /** 投票的綜合 karma（上 - 下）。 **/
    votes: number
    votesUp: number
    votesDown: number
    /** 若使用者在評論時已登入，或他們已驗證該評論，或在留下評論時已驗證其工作階段，則為 true。 **/
    verified: boolean
    /** 評論被驗證的日期。 **/
    verifiedDate?: number
    /** 是否有管理員將此評論標記為已審查。 **/
    reviewed: boolean
    /** 大頭貼的位置，或其 base64 編碼。只有在透過 SSO 傳入該值時，才會是 base64。 **/
    avatarSrc?: string
    /** 評論是否被手動或自動標記為垃圾訊息？ **/
    isSpam: boolean
    /** 評論是否被自動判定為垃圾訊息？ **/
    aiDeterminedSpam: boolean
    /** 評論中是否包含圖片？ **/
    hasImages: boolean
    /** 在「最相關」排序方向下，評論所屬的頁碼。 **/
    pageNumber: number
    /** 在「最舊優先」排序方向下，評論所屬的頁碼。 **/
    pageNumberOF: number
    /** 在「最新優先」排序方向下，評論所屬的頁碼。 **/
    pageNumberNF: number
    /** 評論是自動核准還是人工核准？ **/
    approved: boolean
    /** 撰寫評論時使用者的區域代碼（格式：en_us）。 **/
    locale: string
    /** 評論中被成功解析的 @提及。 **/
    mentions?: CommentUserMention[]
    /** 評論來源的網域。 **/
    domain?: string
    /** 與此評論相關的（選用）審核群組 id 列表。 **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

When users are tagged in a comment, the information is stored in a list called `mentions`. Each object in that list
has the following structure.

[inline-code-attrs-start title = 'Webhook 提及物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 使用者的 id。對於 SSO 使用者，會加上您的租戶 id 前綴。 **/
    id: string
    /** 最終的 @提及 標籤文字，包含 @ 符號。 **/
    tag: string
    /** 原始的 @提及 標籤文字，包含 @ 符號。 **/
    rawTag: string
    /** 被標註的是哪種類型的使用者。user = FastComments.com 帳戶。sso = SSO 使用者。 **/
    type: 'user'|'sso'
    /** 若使用者選擇不接收通知，此欄位仍會被設為 true。 **/
    sent: boolean
}
[inline-code-end]

#### HTTP 方法

您可以在管理面板中為每種 webhook 事件類型設定 HTTP 方法：

- **Create Event**: POST or PUT (default: PUT)
- **Update Event**: POST or PUT (default: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE)

由於所有請求都包含 ID，Create 與 Update 操作預設為冪等（PUT）。重複相同的 Create 或 Update 請求不應在您端產生重複的物件。

#### 請求標頭

每個 webhook 請求都包含下列標頭：

| 標頭 | 說明 |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | Your API Secret |
| `X-FastComments-Timestamp` | Unix timestamp (seconds) when the request was signed |
| `X-FastComments-Signature` | HMAC-SHA256 signature (`sha256=<hex>`) |

See [安全性與 API 令牌](/guides/webhooks/webhooks-api-tokens) for information on verifying the HMAC signature.

---