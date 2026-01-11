---
透過 webhook 傳送的唯一結構是下方 TypeScript 中描述的 WebhookComment 物件。

#### WebhookComment 物件結構

##### 「create」事件結構
「create」事件的請求主體為 WebhookComment 物件。

##### 「update」事件結構
「update」事件的請求主體為 WebhookComment 物件。

##### 「delete」事件結構
「delete」事件的請求主體為 WebhookComment 物件。

    變更自 2023 年 11 月 14 日起
    先前「delete」事件的請求主體僅包含評論 id。現在它包含刪除時的完整評論。


[inline-code-attrs-start title = 'WebhookComment 物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** 評論的 id。 **/
    id: string
    /** 用以識別評論串的 id 或 URL。已正規化。 **/
    urlId: string
    /** 指向留言所在位置的 URL。 **/
    url?: string
    /** 發表評論的使用者 id。若為 SSO，會加上租戶 id 前綴。 **/
    userId?: string
    /** 發表評論者的電子郵件。 **/
    commenterEmail?: string
    /** 顯示在評論小工具中的使用者名稱。使用 SSO 時可能為 displayName。 **/
    commenterName: string
    /** 原始評論文字。 **/
    comment: string
    /** 經解析後的評論文字。 **/
    commentHTML: string
    /** 外部評論 id。 **/
    externalId?: string
    /** 父評論的 id。 **/
    parentId?: string | null
    /** 留言的 UTC 日期。 **/
    date: UTC_ISO_DateString
    /** 投票的合併評分（正票 - 負票）。 **/
    votes: number
    votesUp: number
    votesDown: number
    /** 如果使用者在評論時已登入、或已驗證該評論、或在留言時驗證了其會話，則為 true。 **/
    verified: boolean
    /** 評論被驗證的日期。 **/
    verifiedDate?: number
    /** 是否被管理員標記為已審核。 **/
    reviewed: boolean
    /** 大頭貼的位置或 base64 編碼。僅當 SSO 傳入該值時才會是 base64。 **/
    avatarSrc?: string
    /** 該評論是否被手動或自動標記為垃圾訊息？ **/
    isSpam: boolean
    /** 該評論是否被自動判定為垃圾訊息？ **/
    aiDeterminedSpam: boolean
    /** 評論中是否包含圖片？ **/
    hasImages: boolean
    /** 「最相關」排序方向下評論所在的頁碼。 **/
    pageNumber: number
    /** 「最舊在前」排序方向下評論所在的頁碼。 **/
    pageNumberOF: number
    /** 「最新在前」排序方向下評論所在的頁碼。 **/
    pageNumberNF: number
    /** 該評論是被自動或手動核准？ **/
    approved: boolean
    /** 發表評論時使用者的區域代碼（格式：en_us）。 **/
    locale: string
    /** 評論中成功解析的 @mentions。 **/
    mentions?: CommentUserMention[]
    /** 評論來源的網域。 **/
    domain?: string
    /** 與此評論相關聯的（可選）審核群組 id 清單。 **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

當使用者在評論中被標註時，資訊會儲存在名為 `mentions` 的清單中。該清單中的每個物件具有以下結構。

[inline-code-attrs-start title = 'Webhook 提及物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 使用者 id。若為 SSO 使用者，會加上租戶 id 前綴。 **/
    id: string
    /** 最終的 @mention 標籤文字（包含 @ 符號）。 **/
    tag: string
    /** 原始的 @mention 標籤文字（包含 @ 符號）。 **/
    rawTag: string
    /** 被標註的使用者類型。user = FastComments.com 帳號。sso = SSOUser。 **/
    type: 'user'|'sso'
    /** 若使用者選擇退出通知，此欄仍會被設為 true。 **/
    sent: boolean
}
[inline-code-end]

#### 使用的 HTTP 方法

**Create 與 Update 都使用 HTTP PUT，而非 POST！**

由於我們的所有請求都包含 ID，重複相同的 Create 或 Update 請求不應該在你端建立新的物件。

這表示這些調用是冪等的，根據 HTTP 規範應使用 PUT 事件。

---