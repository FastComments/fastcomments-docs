透過 webhook 傳送的唯一結構是 WebhookComment 物件，以下以 TypeScript 說明。

#### WebhookComment 物件結構

##### 「Create」事件結構
「create」事件的請求主體是一個 WebhookComment 物件。

##### 「Update」事件結構
「update」事件的請求主體是一個 WebhookComment 物件。

##### 「Delete」事件結構
「delete」事件的請求主體是一個 WebhookComment 物件。

    2023 年 11 月 14 日的變更
    先前「delete」事件的請求主體僅包含留言 id。現在則包含刪除當下的完整留言內容。


每個鍵值一律都會出現在請求主體中。當留言的某個欄位沒有值時，主體會帶有 `null`
(布林值為 `false`,列表為 `[]`),因此每次傳送的資料形狀不會因留言不同而改變。

[inline-code-attrs-start title = 'WebhookComment 物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 留言的 id。 **/
    id: string
    /** 標識留言串的 id 或 URL。已標準化。 **/
    urlId: string
    /** 指向留言所在位置的 URL。 **/
    url: string | null
    /** 留言者的使用者 id。若為 SSO,則以租戶 id 為前綴。 **/
    userId: string | null
    /** 留言者的電子郵件。 **/
    commenterEmail: string | null
    /** 在留言小工具中顯示的使用者名稱。使用 SSO 時,可以是 displayName。 **/
    commenterName: string
    /** 原始留言文字。 **/
    comment: string
    /** 解析後的留言文字。 **/
    commentHTML: string
    /** 留言的外部 id。 **/
    externalId: string | null
    /** 父留言的 id。 **/
    parentId: string | null
    /** 留言發表的 UTC 日期。 **/
    date: UTC_ISO_DateString
    /** 投票的合計業力(讚 - 倒讚)。 **/
    votes: number
    votesUp: number
    votesDown: number
    /** 若使用者在留言時已登入、或已驗證留言、或在留言時已驗證其工作階段,則為 true。 **/
    verified: boolean
    /** 留言被驗證的 UTC 日期。 **/
    verifiedDate: UTC_ISO_DateString | null
    /** 若版主已將留言標記為已審閱。 **/
    reviewed: boolean
    /** 頭像的位置或 base64 編碼。僅在 SSO 傳入的值為 base64 時才會是 base64。 **/
    avatarSrc: string | null
    /** 留言是被手動還是自動標記為垃圾訊息? **/
    isSpam: boolean
    /** 留言是否被自動標記為垃圾訊息? **/
    aiDeterminedSpam: boolean
    /** 留言中是否有圖片? **/
    hasImages: boolean
    /** 留言在「最相關」排序方向下所在的頁碼。 **/
    pageNumber: number | null
    /** 留言在「最舊優先」排序方向下所在的頁碼。 **/
    pageNumberOF: number | null
    /** 留言在「最新優先」排序方向下所在的頁碼。 **/
    pageNumberNF: number | null
    /** 留言是被自動還是手動核准? **/
    approved: boolean
    /** 撰寫留言時使用者的語系代碼(格式:en_us)。 **/
    locale: string | null
    /** 留言中成功解析出的 @mentions。若無則為空。 **/
    mentions: CommentUserMention[]
    /** 留言所屬的網域。 **/
    domain: string | null
    /** 與此留言相關聯的審核群組 id。若無則為空。 **/
    moderationGroupIds: string[]
    /** 此留言中留言者名稱旁顯示的徽章,以事件建立當下為準。若無則為空。 **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

當使用者在留言中被標記時,相關資訊會儲存在名為 `mentions` 的列表中。該列表中的每個物件
具有以下結構。

[inline-code-attrs-start title = 'Webhook Mentions 物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 使用者 id。對於 SSO 使用者,此值會以您的租戶 id 為前綴。 **/
    id: string
    /** 最終的 @mention 標籤文字,包含 @ 符號。 **/
    tag: string
    /** 原始的 @mention 標籤文字,包含 @ 符號。 **/
    rawTag: string
    /** 被標記的使用者類型。user = FastComments.com 帳號。sso = SSOUser。 **/
    type: 'user'|'sso'
    /** 若使用者選擇退出通知,此值仍會設為 true。 **/
    sent: boolean
}
[inline-code-end]

`badges` 列表中的每個項目具有以下結構。徽章類型請參閱[徽章指南](/guide-badges.html)。

[inline-code-attrs-start title = 'Webhook Badge 物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** 授予此使用者的徽章 id。 **/
    id: string
    /** 徽章類型。 **/
    type: number
    /** 徽章描述,滑鼠懸停在徽章上時顯示。 **/
    description: string
    /** 徽章上顯示的文字。 **/
    displayLabel?: string | null
    /** 徽章顯示的圖片(若使用的話)。 **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** 套用於徽章的額外 CSS 類別。 **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTP 方法

您可以在管理面板中為每種 webhook 事件類型設定 HTTP 方法:

- **Create 事件**:POST 或 PUT(預設:PUT)
- **Update 事件**:POST 或 PUT(預設:PUT)
- **Delete 事件**:DELETE、POST 或 PUT(預設:DELETE)

由於所有請求都包含一個 ID,Create 與 Update 操作預設即為冪等(PUT)。重複相同的 Create 或 Update 請求不應在您那邊建立重複的物件。

#### 請求標頭

每個 webhook 請求都包含以下標頭:

| 標頭 | 說明 |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | 您的 API Secret |
| `X-FastComments-Timestamp` | 請求簽署時的 Unix 時間戳(秒) |
| `X-FastComments-Signature` | HMAC-SHA256 簽名(`sha256=<hex>`) |

有關驗證 HMAC 簽名的資訊,請參閱[安全性與 API 權杖](/guide-webhooks.html#webhooks-api-tokens)。