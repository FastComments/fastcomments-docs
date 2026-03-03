`UserBadge` 是一个对象，表示分配给 FastComments 系统中用户的徽章。

徽章可以根据用户的活动（例如评论数量、回复时间、资深状态）自动分配，或由站点管理员手动分配。

`UserBadge` 对象的结构如下：

[inline-code-attrs-start title = 'UserBadge 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** 此用户徽章分配的唯一标识符 */
    id: string
    /** 此徽章分配到的用户的 ID */
    userId: string
    /** 来自租户徽章目录的徽章定义的 ID */
    badgeId: string
    /** 创建/分配此徽章的租户的 ID */
    fromTenantId: string
    /** 此徽章创建时间（自纪元以来的毫秒数） */
    createdAt?: number
    /** 用户获得此徽章的时间（自纪元以来的毫秒数） */
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
    /** 对于基于阈值的徽章，阈值数值 */
    threshold?: number
    /** 徽章的名称/标签 */
    name?: string
    /** 徽章的详细描述 */
    description?: string
    /** 徽章上显示的文本 */
    displayLabel?: string
    /** 徽章所显示图像的 URL */
    displaySrc?: string
    /** 徽章的背景颜色（十六进制代码） */
    backgroundColor?: string
    /** 徽章的边框颜色（十六进制代码） */
    borderColor?: string
    /** 徽章的文本颜色（十六进制代码） */
    textColor?: string
    /** 用于样式的额外 CSS 类 */
    cssClass?: string
    /** 对于资深用户徽章，时间阈值（毫秒） */
    veteranUserThresholdMillis?: number
    /** 此徽章是否显示在用户的评论上 */
    displayedOnComments: boolean
    /** 徽章的显示顺序 */
    order?: number
    /** 如果设置，此徽章仅在具有匹配 urlId 的页面上显示。全局徽章为 null。 */
    urlId?: string | null
}
[inline-code-end]
---