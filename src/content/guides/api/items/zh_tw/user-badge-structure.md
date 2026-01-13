`UserBadge` 是一個物件，代表分配給 FastComments 系統中使用者的徽章。

徽章可以根據使用者的活動自動分配（例如留言數、回覆時間、資深使用者狀態），或由網站管理員手動分配。

以下為 `UserBadge` 物件的結構：

[inline-code-attrs-start title = 'UserBadge 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** 此使用者徽章分配的唯一識別碼 */
    id: string
    /** 此徽章所指派之使用者的 ID */
    userId: string
    /** 來自租戶徽章目錄的徽章定義 ID */
    badgeId: string
    /** 建立/指派此徽章的租戶 ID */
    fromTenantId: string
    /** 此徽章建立時間（以 epoch 起算的毫秒） */
    createdAt?: number
    /** 使用者收到此徽章的時間（以 epoch 起算的毫秒） */
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
    /** 對於以門檻為條件的徽章，門檻值 */
    threshold?: number
    /** 徽章的名稱/標籤 */
    name?: string
    /** 徽章的詳細描述 */
    description?: string
    /** 徽章上顯示的文字 */
    displayLabel?: string
    /** 徽章上顯示的圖片 URL */
    displaySrc?: string
    /** 徽章的背景顏色（十六進位碼） */
    backgroundColor?: string
    /** 徽章的邊框顏色（十六進位碼） */
    borderColor?: string
    /** 徽章的文字顏色（十六進位碼） */
    textColor?: string
    /** 用於樣式的額外 CSS 類別 */
    cssClass?: string
    /** 對於資深徽章，時間門檻（毫秒） */
    veteranUserThresholdMillis?: number
    /** 此徽章是否顯示在使用者的留言上 */
    displayedOnComments: boolean
    /** 徽章的顯示順序 */
    order?: number
}
[inline-code-end]
---