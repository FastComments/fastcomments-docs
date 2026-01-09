`UserBadgeProgress` 是一個物件，表示使用者在 FastComments 系統中獲取各種徽章的進度。

此追蹤有助於根據使用者在您的社群中的活動與參與來決定何時應該自動授予徽章。

`UserBadgeProgress` 物件的結構如下：

[inline-code-attrs-start title = 'UserBadgeProgress 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** 此進度記錄的唯一識別碼 */
    id: string
    /** 此進度記錄所屬的租戶 ID */
    tenantId: string
    /** 此進度記錄所追蹤的使用者 ID */
    userId: string
    /** 使用者在系統中的第一則評論的 ID */
    firstCommentId?: string
    /** 使用者第一則評論的日期（自 epoch 起的毫秒數） */
    firstCommentDate?: number
    /** 根據使用者活動自動計算的信任係數 */
    autoTrustFactor?: number
    /** 由管理員手動設定的信任係數 */
    manualTrustFactor?: number
    /** 包含各種指標的詳細進度物件，鍵與 BadgeType enum 相符 */
    progress: {
        /** 0: CommentCount - 使用者發表評論的數量 */
        '0'?: number
        /** 1: CommentUpVotes - 使用者收到的讚數 */
        '1'?: number
        /** 2: CommentReplies - 使用者發表的回覆數 */
        '2'?: number
        /** 3: CommentsPinned - 使用者被固定的評論數 */
        '3'?: number
        /** 4: Veteran - 使用者帳戶年齡 */
        '4'?: number
        /** 5: NightOwl - 使用者在夜間時段發文的次數 */
        '5'?: number
        /** 6: FastReplyTime - 平均回覆時間（毫秒） */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - 對於管理員徽章，刪除的評論數 */
        '7'?: number
        /** 8: ModeratorCommentsApproved - 對於管理員徽章，核准的評論數 */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - 對於管理員徽章，未核准的評論數 */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - 對於管理員徽章，已審核的評論數 */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - 對於管理員徽章，被標記為垃圾評論的數量 */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - 對於管理員徽章，被標記為非垃圾評論的數量 */
        '12'?: number
        /** 13: RepliedToSpecificPage - 每個頁面的回覆數量 */
        '13'?: Record<string, number>
    }
}
[inline-code-end]