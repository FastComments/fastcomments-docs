`UserBadge` 是一個物件，代表在 FastComments 系統中指派給使用者的徽章。

徽章可以根據使用者的活動自動指派（例如留言數、回覆時間、資深狀態），也可以由網站管理員手動指派。

`UserBadge` 物件的結構如下：

[inline-code-attrs-start title = 'UserBadge 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** 此使用者徽章指派的唯一識別碼 */
    id: string
    /** 此徽章指派給的使用者 ID */
    userId: string
    /** 租戶徽章目錄中的徽章定義 ID */
    badgeId: string
    /** 建立/指派此徽章的租戶 ID */
    fromTenantId: string
    /** 此徽章的建立時間（自 epoch 起的毫秒數） */
    createdAt?: number
    /** 使用者收到此徽章的時間（自 epoch 起的毫秒數） */
    receivedAt?: number
    /** 
     * 徽章類型： 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** 對於基於門檻的徽章，此為門檻值 */
    threshold?: number
    /** 徽章的名稱/標籤 */
    name?: string
    /** 徽章的詳細說明 */
    description?: string
    /** 徽章上顯示的文字 */
    displayLabel?: string
    /** 徽章上顯示圖片的 URL */
    displaySrc?: string
    /** 徽章的背景顏色（十六進位碼） */
    backgroundColor?: string
    /** 徽章的邊框顏色（十六進位碼） */
    borderColor?: string
    /** 徽章的文字顏色（十六進位碼） */
    textColor?: string
    /** 用於樣式的額外 CSS 類別 */
    cssClass?: string
    /** 對於資深使用者徽章，此為時間門檻（毫秒） */
    veteranUserThresholdMillis?: number
    /** 此徽章是否顯示在使用者的留言上 */
    displayedOnComments: boolean
    /** 徽章的顯示順序 */
    order?: number
    /** 若有設定，此徽章僅顯示在符合 urlId 的頁面上。全域徽章則為 Null。 */
    urlId?: string | null
}
[inline-code-end]