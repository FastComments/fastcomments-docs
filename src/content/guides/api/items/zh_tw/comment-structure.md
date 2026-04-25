---
一個 `Comment` 物件代表使用者留下的一則評論。

父評論與子評論之間的關聯是透過 `parentId` 定義的。

Comment 物件的結構如下：

[inline-code-attrs-start title = '評論結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** 唯讀: 如果垃圾郵件引擎判定該評論為垃圾郵件，則設為 true。 **/
    aiDeterminedSpam?: boolean
    /** 該評論是否被核准顯示。儲存評論時設為 true，否則將被隱藏。 **/
    approved?: boolean
    /** 使用者的大頭貼。 **/
    avatarSrc?: string
    /** 子評論。並非在所有情況下都有填入。當透過 API 將 asTree 設為 true 時會使用。 **/
    children: Comment[]
    /** 使用者輸入的原始評論內容。 **/
    comment: string
    /** 唯讀: 使用者的評論解析後的 HTML。 **/
    commentHTML?: string
    /** 使用者的電子郵件。若不允許匿名評論則為必填。 **/
    commenterEmail?: string
    /** 使用者的連結（例如，他們的部落格）。 **/
    commenterLink?: string
    /** 使用者的名稱。總是必填。如果沒有可用名稱，請設為像 "Anonymous" 之類的值。 **/
    commenterName: string
    /** 留言的日期，使用 UTC epoch。 **/
    date: number
    /** 評論的「顯示標籤」，例如 "Admin"、"Moderator" 或類似 "VIP User" 的標示。 **/
    displayLabel?: string
    /** 留言發布的網域。 **/
    domain?: string
    /** 唯讀: 該評論被檢舉的次數。 **/
    flagCount?: number
    /** 評論中成功解析出的 #hashtags。你也可以手動新增 hashtags 以供查詢，但它們不會自動顯示在評論文字中。 **/
    hashTags?: CommentHashTag[]
    /** 唯讀: 評論是否包含圖片？ **/
    hasImages?: boolean
    /** 唯讀: 評論是否包含連結？ **/
    hasLinks?: boolean
    /** 唯讀: 唯一的評論 id。 **/
    id: string
    /** 僅在建立時使用！此值會被雜湊後儲存。 **/
    ip?: string
    /** 唯讀: 目前使用者是否封鎖了撰寫此評論的使用者？ **/
    isBlocked?: boolean
    /** 唯讀: 此評論是否由管理員所發？會自動根據 userId 設定。 **/
    isByAdmin?: boolean
    /** 唯讀: 此評論是否由版主所發？會自動根據 userId 設定。 **/
    isByModerator?: boolean
    /** 若評論被軟刪除（由於其他設定必須保留佔位符），則設為 true。 **/
    isDeleted?: boolean
    /** 若使用者帳號被刪除且評論需被保留，則設為 true。 **/
    isDeletedUser?: boolean
    /** 唯讀: 該評論是否已被目前登入使用者（contextUserId）檢舉？ **/
    isFlagged?: boolean
    /** 該評論是否被置頂？ **/
    isPinned?: boolean
    /** 該評論是否被鎖定？若為 true，則沒有人（包括版主）能回覆、編輯或刪除該評論，直到解除鎖定。 **/
    isLocked?: boolean
    /** 該評論是否為垃圾評論？ **/
    isSpam?: boolean
    /** 唯讀: 該評論在目前使用者（contextUserId）是否被投下倒票？ **/
    isVotedDown?: boolean
    /** 唯讀: 該評論在目前使用者（contextUserId）是否被投上票？ **/
    isVotedUp?: boolean
    /** 評論所使用的語系。如果未提供，將從 HTTP 的 Accept-Language 標頭推斷。 **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** 唯讀: 評論中成功解析出的 @mentions。 **/
    mentions?: CommentUserMention[]
    /** 與評論相關的選用 metadata。 **/
    meta?: Record<string, string | number | boolean>
    /** 與此評論相關的選用審核群組 id 清單。 **/
    moderationGroupIds?: string[]|null
    /** 唯讀: 對應目前使用者（contextUserId）對此評論投票的 vote 物件 id。 **/
    myVoteId?: string
    /** 是否已為此評論發送給評論者通知。若要在匯入時避免發送通知，請設為 true。 **/
    notificationSentForParent?: boolean
    /** 是否已為此評論發送給租戶使用者通知。若要在匯入時避免發送通知，請設為 true。 **/
    notificationSentForParentTenant?: boolean
    /** 此評論所在頁面的標題。 **/
    pageTitle?: string
    /** 若我們在回覆一則評論，這是我們所回覆的評論 ID。 **/
    parentId?: string|null
    /** 該評論是否被標記為已審核。 **/
    reviewed: boolean
    /** 該評論所屬的租戶 id。 **/
    tenantId: string
    /** 撰寫該評論的使用者。當使用名稱/電子郵件儲存評論時會自動建立。 **/
    userId?: string|null
    /** 此評論可見位置的 URL，例如部落格文章。 **/
    url: string
    /** 你傳給我們的 urlId 的「清理」版本。儲存時你會指定此欄位，但當你取回評論時，此欄位會被「清理」，你的原始值會移到 "urlIdRaw"。 **/
    urlId: string
    /** 唯讀: 你傳給我們的原始 urlId。 **/
    urlIdRaw?: string
    /** 使用者與此評論是否已通過驗證？ **/
    verified: boolean
    /** 被投上的票數。 **/
    votesUp?: number
    /** 被投下的票數。 **/
    votesDown?: number
    /** 評論的「聲望」(= votes up - votes down)。 **/
    votes?: number
}
[inline-code-end]

有些欄位被標示為 `READONLY` — 這些欄位由 API 回傳但無法設定。

### 評論文字結構

評論是使用 FastComments 的一種 Markdown 變體撰寫，這就是 Markdown 外加傳統 bbcode 風格的圖片標籤，例如 `[img]path[/img]`。

文字儲存在兩個欄位。使用者輸入的原始文字會不作修改地儲存在 `comment` 欄位。這會被渲染並儲存在 `commentHTML` 欄位。

允許的 HTML 標籤為 `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`。

建議渲染該 HTML，因為它是非常小的 HTML 子集，建立一個渲染器相當簡單。舉例來說，針對 React Native 與 Flutter 有多個函式庫可以協助。

你也可以選擇渲染 `comment` 欄位未正規化的值。 [範例解析器在這裡。](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js)

這個範例解析器也可以調整以處理 HTML，並將 HTML 標籤轉換成你平台上預期渲染的元素。

### 標註

當使用者在評論中被標註時，資訊會儲存在名為 `mentions` 的清單中。該清單中的每個物件具有以下結構。

[inline-code-attrs-start title = '評論提及物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 使用者的 id。對於 SSO 使用者，會加上你的租戶 id 做為前綴。 **/
    id: string
    /** 最終的 @mention 標籤文字，包括 @ 符號。 **/
    tag: string
    /** 原始的 @mention 標籤文字，包括 @ 符號。 **/
    rawTag: string
    /** 被標註的使用者類型。user = FastComments.com 帳號。sso = SSOUser。 **/
    type: 'user'|'sso'
    /** 若使用者選擇不接收通知，此欄位仍會被設為 true（表示已送出）。 **/
    sent: boolean
}
[inline-code-end]

### 主題標籤

當使用 hashtag 並成功解析時，資訊會儲存在名為 `hashTags` 的清單中。該清單中的每個物件具有以下結構。若設定了 `retain`，也可以手動將 hashtags 新增到評論的 `hashTags` 陣列以供查詢。

[inline-code-attrs-start title = '評論標籤物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** hashtag 的 id。 **/
    id: string
    /** 最終的 #hashtag 標籤文字，包括 # 符號。 **/
    tag: string
    /** 若該 hashtag 與自訂 URL 關聯，則會定義此欄位。 **/
    url?: string
    /** 若在評論更新時應保留該 hashtag（即使它不存在於評論文字中），可設此欄位為 true。對於在不更改評論文字的情況下標註評論很有用。 **/
    retain?: boolean
}
[inline-code-end]

---