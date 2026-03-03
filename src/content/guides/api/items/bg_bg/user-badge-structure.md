---
`UserBadge` е обект, който представлява значка, присвоена на потребител в системата FastComments.

Значките могат да бъдат присвоявани на потребителите автоматично въз основа на тяхната активност (например брой коментари, време за отговор, статус на ветеран) или ръчно от администраторите на сайта.

Структурата на обекта `UserBadge` е следната:

[inline-code-attrs-start title = 'Структура на UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Уникален идентификатор за това присвояване на значка на потребител */
    id: string
    /** ID of the user this badge is assigned to */
    userId: string
    /** ID на дефиницията на значката от каталога със значки на наемателя */
    badgeId: string
    /** ID на наемателя, който е създал/присвоил тази значка */
    fromTenantId: string
    /** Кога е създадена тази значка (милисекунди от епохата) */
    createdAt?: number
    /** Кога потребителят е получил тази значка (милисекунди от епохата) */
    receivedAt?: number
    /** 
     * Тип на значката: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** За значки, базирани на праг, стойността на прага */
    threshold?: number
    /** Името/етикетът на значката */
    name?: string
    /** Подробно описание на значката */
    description?: string
    /** Текстът, показван на значката */
    displayLabel?: string
    /** URL към изображение, показвано на значката */
    displaySrc?: string
    /** Фонов цвят за значката (hex код) */
    backgroundColor?: string
    /** Цвят на рамката за значката (hex код) */
    borderColor?: string
    /** Цвят на текста за значката (hex код) */
    textColor?: string
    /** Допълнителен CSS клас за стилизиране */
    cssClass?: string
    /** За ветерански значки, праговото време в милисекунди */
    veteranUserThresholdMillis?: number
    /** Дали тази значка се показва върху коментарите на потребителя */
    displayedOnComments: boolean
    /** Ред на показване на значката */
    order?: number
    /** If set, this badge is only displayed on the page with the matching urlId. Null for global badges. */
    urlId?: string | null
}
[inline-code-end]
---