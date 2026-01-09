`UserBadge` је објекат који представља ознаку додељену кориснику у FastComments систему.

Ознаке могу бити аутоматски додељене корисницима на основу њихове активности (као што су број коментара, време одговора, статус ветерана) или ручно од стране администратора сајта.

Структура објекта `UserBadge` је следећа:

[inline-code-attrs-start title = 'Структура UserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface UserBadge {
    /** Јединствени идентификатор ове додјеле ознаке кориснику */
    id: string
    /** ID корисника којем је ова ознака додељена */
    userId: string
    /** ID дефиниције ознаке из каталога ознака тенанта */
    badgeId: string
    /** ID тенанта који је креирао/доделио ову ознаку */
    fromTenantId: string
    /** Када је ова ознака креирана (милисекунде од епохе) */
    createdAt?: number
    /** Када је корисник примио ову ознаку (милисекунде од епохе) */
    receivedAt?: number
    /** 
     * Тип ознаке: 
     * 0=CommentCount, 1=CommentUpVotes, 2=CommentReplies, 3=CommentsPinned, 
     * 4=Veteran, 5=NightOwl, 6=FastReplyTime, 7=ModeratorCommentsDeleted,
     * 8=ModeratorCommentsApproved, 9=ModeratorCommentsUnapproved, 10=ModeratorCommentsReviewed,
     * 11=ModeratorCommentsMarkedSpam, 12=ModeratorCommentsMarkedNotSpam, 13=RepliedToSpecificPage,
     * 14=Manual
     */
    type: number
    /** За ознаке засноване на прагу, вредност прага */
    threshold?: number
    /** Назив/натпис ознаке */
    name?: string
    /** Детаљан опис ознаке */
    description?: string
    /** Текст који се приказује на ознаци */
    displayLabel?: string
    /** URL до слике која се приказује на ознаци */
    displaySrc?: string
    /** Боја позадине ознаке (хекс код) */
    backgroundColor?: string
    /** Боја границе ознаке (хекс код) */
    borderColor?: string
    /** Боја текста на ознаци (хекс код) */
    textColor?: string
    /** Додатна CSS класа за стилизацију */
    cssClass?: string
    /** За ветеранске ознаке, временски праг у милисекундама */
    veteranUserThresholdMillis?: number
    /** Да ли се ова ознака приказује на корисниковим коментарима */
    displayedOnComments: boolean
    /** Редослед приказа ознаке */
    order?: number
}
[inline-code-end]