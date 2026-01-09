`UserBadge` 是一个对象，表示分配给 FastComments 系统中用户的徽章。

徽章可以根据用户的活动（例如评论数量、回复时间、资深用户状态）自动分配，也可以由站点管理员手动分配。

以下是 `UserBadge` 对象的结构：

[inline-code-attrs-start title = 'UserBadge 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** 此用户徽章分配的唯一标识符 */
    id: string
    /** 该徽章分配给的用户 ID */
    userId: string
    /** 来自租户徽章目录的徽章定义 ID */
    badgeId: string
    /** 创建/分配此徽章的租户 ID */
    fromTenantId: string
    /** 此徽章的创建时间（自纪元以来的毫秒） */
    createdAt?: number
    /** 用户收到此徽章的时间（自纪元以来的毫秒） */
    receivedAt?: number
    /** 
     * 徽章类型： 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** 对于基于阈值的徽章，阈值 */
    threshold?: number
    /** 徽章的名称/标签 */
    name?: string
    /** 徽章的详细描述 */
    description?: string
    /** 徽章上显示的文字 */
    displayLabel?: string
    /** 徽章上显示图像的 URL */
    displaySrc?: string
    /** 徽章的背景颜色（十六进制代码） */
    backgroundColor?: string
    /** 徽章的边框颜色（十六进制代码） */
    borderColor?: string
    /** 徽章的文字颜色（十六进制代码） */
    textColor?: string
    /** 用于样式的额外 CSS 类 */
    cssClass?: string
    /** 对于资深徽章，时间阈值（毫秒） */
    veteranUserThresholdMillis?: number
    /** 此徽章是否显示在用户的评论上 */
    displayedOnComments: boolean
    /** 徽章的显示顺序 */
    order?: number
}
[inline-code-end]
---