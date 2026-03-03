`UserBadge` је објекат који представља значку додељену кориснику у систему FastComments.

Значке могу бити додељене корисницима аутоматски на основу њихове активности (као што су број коментара, време одговора, статус Veteran) или ручно од стране администратора сајта.

Структура `UserBadge` објекта је следећа:

[inline-code-attrs-start title = 'Структура објекта UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Јединствени идентификатор за додељу ове значке кориснику */
    id: string
    /** ID корисника коме је ова значка додељена */
    userId: string
    /** ID дефиниције значке из каталога значки тенанта */
    badgeId: string
    /** ID тенанта који је креирао/доделио ову значку */
    fromTenantId: string
    /** Када је ова значка креирана (милисекунде од епохе) */
    createdAt?: number
    /** Када је корисник примио ову значку (милисекунде од епохе) */
    receivedAt?: number
    /** 
     * Тип значке: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** За значке засноване на прагу, вредност прага */
    threshold?: number
    /** Име/ознака значке */
    name?: string
    /** Детаљан опис значке */
    description?: string
    /** Текст који се приказује на значки */
    displayLabel?: string
    /** URL до слике која се приказује на значки */
    displaySrc?: string
    /** Боја позадине за значку (хекс код) */
    backgroundColor?: string
    /** Боја границе значке (хекс код) */
    borderColor?: string
    /** Боја текста значке (хекс код) */
    textColor?: string
    /** Додатна CSS класа за стилизацију */
    cssClass?: string
    /** За значке типа Veteran, временски праг у милисекундама */
    veteranUserThresholdMillis?: number
    /** Да ли се ова значка приказује на коментарима корисника */
    displayedOnComments: boolean
    /** Редослед приказа значке */
    order?: number
    /** Ако је подешено, ова значка се приказује само на страници са одговарајућим urlId. Null за глобалне значке. */
    urlId?: string | null
}
[inline-code-end]