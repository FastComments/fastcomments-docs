`UserBadgeProgress` 是一个对象，表示用户在 FastComments 系统中获取各种徽章的进度。

此跟踪有助于确定何时应根据用户在社区的活动和参与自动授予徽章。

`UserBadgeProgress` 对象的结构如下：

[inline-code-attrs-start title = 'UserBadgeProgress 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadgeProgress {
    /** 此进度记录的唯一标识符 */
    id: string
    /** 此进度记录所属租户的 ID */
    tenantId: string
    /** 此进度记录所跟踪的用户的 ID */
    userId: string
    /** 用户在系统中的第一条评论的 ID */
    firstCommentId?: string
    /** 用户第一条评论的日期（自纪元以来的毫秒数） */
    firstCommentDate?: number
    /** 基于用户活动自动计算的信任因子 */
    autoTrustFactor?: number
    /** 由管理员手动设置的信任因子 */
    manualTrustFactor?: number
    /** 包含各种指标的详细进度对象，键与 BadgeType 枚举匹配 */
    progress: {
        /** 0: CommentCount - 用户发布的评论数量 */
        '0'?: number
        /** 1: CommentUpVotes - 用户收到的赞数 */
        '1'?: number
        /** 2: CommentReplies - 用户发布的回复数量 */
        '2'?: number
        /** 3: CommentsPinned - 用户的被置顶评论数量 */
        '3'?: number
        /** 4: Veteran - 用户账户年龄 */
        '4'?: number
        /** 5: NightOwl - 用户在夜间时段发布的次数 */
        '5'?: number
        /** 6: FastReplyTime - 平均回复时间（以毫秒为单位） */
        '6'?: number
        /** 7: ModeratorCommentsDeleted - 针对版主徽章，已删除评论的数量 */
        '7'?: number
        /** 8: ModeratorCommentsApproved - 针对版主徽章，已通过审核的评论数量 */
        '8'?: number
        /** 9: ModeratorCommentsUnapproved - 针对版主徽章，未批准的评论数量 */
        '9'?: number
        /** 10: ModeratorCommentsReviewed - 针对版主徽章，已审核的评论数量 */
        '10'?: number
        /** 11: ModeratorCommentsMarkedSpam - 针对版主徽章，被标记为垃圾评论的数量 */
        '11'?: number
        /** 12: ModeratorCommentsMarkedNotSpam - 针对版主徽章，被标记为非垃圾评论的数量 */
        '12'?: number
        /** 13: RepliedToSpecificPage - 针对每个页面的回复数量 */
        '13'?: Record<string, number>
    }
}
[inline-code-end]