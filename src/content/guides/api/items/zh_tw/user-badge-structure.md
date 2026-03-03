`UserBadge` 是一個物件，表示在 FastComments 系統中指派給使用者的徽章。

徽章可以根據使用者活動自動指派（例如評論數、回覆時間、資深使用者狀態），或由網站管理員手動指派。

The structure for the `UserBadge` object is as follows:

[inline-code-attrs-start title = 'UserBadge 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** 此使用者徽章指派的唯一識別碼 */
    id: string
    /** 此徽章所指派使用者的 ID */
    userId: string
    /** 來自租戶徽章目錄的徽章定義 ID */
    badgeId: string
    /** 建立/指派此徽章的租戶 ID */
    fromTenantId: string
    /** 此徽章建立的時間（自 epoch 起的毫秒） */
    createdAt?: number
    /** 使用者收到此徽章的時間（自 epoch 起的毫秒） */
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
    /** 對於基於門檻的徽章，門檻值 */
    threshold?: number
    /** 徽章的名稱/標籤 */
    name?: string
    /** 徽章的詳細描述 */
    description?: string
    /** 顯示在徽章上的文字 */
    displayLabel?: string
    /** 顯示在徽章上的圖片 URL */
    displaySrc?: string
    /** 徽章的背景顏色（十六進位碼） */
    backgroundColor?: string
    /** 徽章的邊框顏色（十六進位碼） */
    borderColor?: string
    /** 徽章文字的顏色（十六進位碼） */
    textColor?: string
    /** 用於樣式的額外 CSS 類別 */
    cssClass?: string
    /** 對於資深徽章，時間門檻（毫秒） */
    veteranUserThresholdMillis?: number
    /** 是否在使用者的評論上顯示此徽章 */
    displayedOnComments: boolean
    /** 徽章的顯示順序 */
    order?: number
    /** 若設定，則此徽章僅在具有相符 urlId 的頁面上顯示。全域徽章為 null。 */
    urlId?: string | null
}
[inline-code-end]
---