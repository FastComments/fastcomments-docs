一個 `Comment` 物件代表使用者留下的評論。

父子評論之間的關係由 `parentId` 定義。

Comment 物件的結構如下：

[inline-code-attrs-start title = '評論結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** 唯讀：如果垃圾郵件引擎判定此評論為垃圾郵件，則設為 true。 **/
    aiDeterminedSpam?: boolean
    /** 評論是否核准顯示。儲存評論時設為 true，否則會被隱藏。 **/
    approved?: boolean
    /** 使用者的大頭貼。 **/
    avatarSrc?: string
    /** 子評論。並非在所有情況下皆會填入。當透過 API 將 asTree 設為 true 時使用。 **/
    children: Comment[]
    /** 評論者的原始評論文字。 **/
    comment: string
    /** 唯讀：評論者的評論解析後的 HTML。 **/
    commentHTML?: string
    /** 評論者的電子郵件。若匿名評論被關閉則為必填。 **/
    commenterEmail?: string
    /** 評論者的連結（例如，他們的部落格）。 **/
    commenterLink?: string
    /** 評論者的名稱。始終為必填。如果沒有，可設為像 "Anonymous" 之類的名稱。 **/
    commenterName: string
    /** 留下評論的日期，以 UTC epoch 表示。 **/
    date: number
    /** 評論的「顯示標籤」，例如 "Admin"、"Moderator"，或像 "VIP User" 之類的。 **/
    displayLabel?: string
    /** 發表評論的網域。 **/
    domain?: string
    /** 唯讀：評論被檢舉的次數。 **/
    flagCount?: number
    /** 評論中成功解析的 #hashtag。你也可以手動加入 hashtag 以供查詢，但它們不會自動顯示在評論文字中。 **/
    hashTags?: CommentHashTag[]
    /** 唯讀：評論是否包含圖片？ **/
    hasImages?: boolean
    /** 唯讀：評論是否包含連結？ **/
    hasLinks?: boolean
    /** 唯讀：唯一的評論 id。 **/
    id: string
    /** 僅在建立時使用！此欄位會被雜湊後儲存。 **/
    ip?: string
    /** 唯讀：目前使用者是否封鎖了撰寫此評論的使用者？ **/
    isBlocked?: boolean
    /** 唯讀：此評論是否由管理員發表？會根據 userId 自動設定。 **/
    isByAdmin?: boolean
    /** 唯讀：此評論是否由版主發表？會根據 userId 自動設定。 **/
    isByModerator?: boolean
    /** 若評論已被軟刪除（因其他設定需保留代表項），則設為 true。 **/
    isDeleted?: boolean
    /** 若使用者帳戶被刪除但評論需被保留，則設為 true。 **/
    isDeletedUser?: boolean
    /** 唯讀：是否已被當前登入使用者（contextUserId）檢舉？ **/
    isFlagged?: boolean
    /** 評論是否被置頂？ **/
    isPinned?: boolean
    /** 評論是否鎖定以禁止新回覆（版主仍可回覆）？ **/
    isLocked?: boolean
    /** 此評論是否為垃圾郵件？ **/
    isSpam?: boolean
    /** 唯讀：對於當前使用者（contextUserId），此評論是否被投下反對票？ **/
    isVotedDown?: boolean
    /** 唯讀：對於當前使用者（contextUserId），此評論是否被投下贊成票？ **/
    isVotedUp?: boolean
    /** 評論所使用的語系。若未提供，將從 Accept-Language HTTP 標頭推斷。 **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** 唯讀：評論中成功解析的 @提及。 **/
    mentions?: CommentUserMention[]
    /** 與評論相關的可選額外資訊（metadata）。 **/
    meta?: Record<string, string | number | boolean>
    /** 與此評論相關的可選 moderation 群組 id 清單。 **/
    moderationGroupIds?: string[]|null
    /** 唯讀：對應於當前使用者（contextUserId）在此評論上投票的 vote 物件 id。 **/
    myVoteId?: string
    /** 是否已針對評論者發送此評論的通知。為了在匯入時避免發送通知，請將此設為 true。 **/
    notificationSentForParent?: boolean
    /** 是否已針對租戶使用者發送此評論的通知。為了在匯入時避免發送通知，請將此設為 true。 **/
    notificationSentForParentTenant?: boolean
    /** 此評論所屬頁面的標題。 **/
    pageTitle?: string
    /** 若為回覆某則評論，這是被回覆的評論 ID。 **/
    parentId?: string|null
    /** 評論是否被標示為已審核。 **/
    reviewed: boolean
    /** 評論所屬的租戶 id。 **/
    tenantId: string
    /** 撰寫評論的使用者。當以名稱/電子郵件儲存評論時會自動建立。 **/
    userId?: string|null
    /** 此評論可見的對應位置 URL，例如部落格文章。 **/
    url: string
    /** 你傳入的 urlId 經過「清理」後的版本。儲存時你會指定此欄位，但當取回評論時，這會是「清理」後的值，而你原本的值會移到 urlIdRaw。 **/
    urlId: string
    /** 唯讀：你傳入的原始 urlId。 **/
    urlIdRaw?: string
    /** 使用者及此評論是否已驗證？ **/
    verified: boolean
    /** 讚成票數。 **/
    votesUp?: number
    /** 反對票數。 **/
    votesDown?: number
    /** 評論的「Karma」（= 讚成票數 - 反對票數）。 **/
    votes?: number
}
[inline-code-end]

Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.

### 評論文字結構

評論以 FastComments 風格的 Markdown 撰寫，這就是 Markdown 加上傳統的 `bbcode` 風格圖片標籤，例如 `[img]path[/img]`。

文字會儲存在兩個欄位。使用者輸入的文字以未修改的形式儲存在 `comment` 欄位；渲染後的內容則儲存在 `commentHTML` 欄位。

允許的 HTML 標籤有 `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br`。

建議將 HTML 渲染出來，因為它只是 HTML 的一個非常小的子集，構建渲染器相當直接。舉例來說，有多個可用於 React Native 與 Flutter 的函式庫可以協助完成這件事

你也可以選擇渲染 `comment` 欄位未標準化的值。[範例解析器在這裡。](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js)。

該範例解析器也可以調整以支援 HTML，並將 HTML 標籤轉換為你平台期待渲染的元素。 

### 標註

當使用者在評論中被標註時，相關資訊會儲存在名為 `mentions` 的清單中。該清單中的每個物件具有以下結構。

[inline-code-attrs-start title = '評論提及物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** 使用者 id。對於 SSO 使用者，會加上你的租戶 id 前綴。 **/
    id: string
    /** 最終的 @提及 標籤文字，包含 @ 符號。 **/
    tag: string
    /** 原始的 @提及 標籤文字，包含 @ 符號。 **/
    rawTag: string
    /** 被標註的使用者類型。user = FastComments.com 帳戶。sso = SSOUser。 **/
    type: 'user'|'sso'
    /** 即使使用者選擇不接收通知，這仍會被設為 true。 **/
    sent: boolean
}
[inline-code-end]

### HashTags

當使用 hashtag 並成功解析時，相關資訊會儲存在名為 `hashTags` 的清單中。該清單中的每個物件具有以下結構。若將 `retain` 設定為 true，hashtag 也可以手動加入至評論的 `hashTags` 陣列以供查詢。

[inline-code-attrs-start title = '評論 Hashtag 物件'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** hashtag id。 **/
    id: string
    /** 最終的 #hashtag 標籤文字，包含 # 符號。 **/
    tag: string
    /** 若該 hashtag 與自訂 URL 關聯，則會定義此欄。 **/
    url?: string
    /** 當評論更新時，若需保留該 hashtag（即使它不存在於評論文字中），則設定此欄。這對於在不更改評論文字的情況下標註評論很有用。 **/
    retain?: boolean
}
[inline-code-end]

---